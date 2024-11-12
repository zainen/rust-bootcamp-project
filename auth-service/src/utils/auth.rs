use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use color_eyre::eyre::{eyre, Context, ContextCompat, Report, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{domain::email::Email, store::BannedTokenStoreType};

use super::constants::{JWT_COOKIE_NAME, JWT_SECRET};

pub const TOKEN_TTL_SECONDS: i64 = 600;

#[derive(Debug, Error)]
pub enum GenerateTokenError {
    #[error("Token Error")]
    TokenError(jsonwebtoken::errors::Error),
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[tracing::instrument(name = "create_token", skip_all)]
fn create_token(claims: &Claims) -> Result<String> {
    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .wrap_err("Failed to create token")
}

#[tracing::instrument(name = "generate_auth_token", skip_all)]
fn generate_auth_token(email: &Email) -> Result<String> {
    let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
        .wrap_err("Failed to create 10 minute time delta")?;

    // create JWT experation time
    let exp = Utc::now()
        .checked_add_signed(delta)
        .wrap_err("Failed to add 10 minutes to current time")?
        .timestamp();

    // cast exp to usize
    let exp: usize = exp
        .try_into()
        .wrap_err(format!("failed to cast i64 into usize. exp time: {}", exp))?;

    let sub = email.as_ref().to_owned();

    let claims = Claims { sub, exp };

    create_token(&claims)
}

#[tracing::instrument(name = "validate_token", skip_all)]
pub async fn validate_token(banned_tokens: &BannedTokenStoreType, token: &str) -> Result<Claims> {
    match banned_tokens.read().await.verify_token_exists(token).await {
        Err(e) => return Err(e.into()),
        Ok(value) => {
            if value {
                return Err(eyre!("Banned token found used"));
            }
        }
    }
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .wrap_err("Failed to decode claims")
}

#[tracing::instrument(name = "create_auth_cookie", skip_all)]
fn create_auth_cookie(token: String) -> Cookie<'static> {
    let cookie = Cookie::build((JWT_COOKIE_NAME, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    cookie
}

#[tracing::instrument(name = "generate_auth_cookie", skip_all)]
pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use tokio::sync::RwLock;

    use crate::domain::BannedTokenStore;

    use super::*;

    #[tokio::test]
    async fn test_generate_auth_cookie() {
        let email = Email::parse("test@test.com".to_owned()).unwrap();

        let cookie = generate_auth_cookie(&email).unwrap();

        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value().split('.').count(), 3);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[tokio::test]
    async fn test_create_auth_cookie() {
        let token = "test_token".to_owned();
        let cookie = create_auth_cookie(token.clone());
        assert_eq!(cookie.name(), JWT_COOKIE_NAME);
        assert_eq!(cookie.value(), token);
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    // #[tokio::test]
    // async fn test_valildate_token_with_valid_token() {
    //     let banned_token_store: BannedTokenStoreType =
    //         Arc::new(RwLock::new(HashmapBannedTokenStore {
    //             tokens: HashMap::new(),
    //         })) as Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;
    //
    //     let email = Email::parse("test@test.com".to_owned()).unwrap();
    //     let token = generate_auth_token(&email).unwrap();
    //     let result = validate_token(&banned_token_store, &token).await.unwrap();
    //     assert_eq!(result.sub, "test@test.com");
    //
    //     let exp = Utc::now()
    //         .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
    //         .expect("valid timestamp")
    //         .timestamp();
    //
    //     assert!(result.exp > exp as usize);
    // }

    // #[tokio::test]
    // async fn test_valildate_token_attempt_same_token() {
    //     let banned_token_store: BannedTokenStoreType =
    //         Arc::new(RwLock::new(HashmapBannedTokenStore {
    //             tokens: HashMap::new(),
    //         })) as Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;
    //     let mut store = banned_token_store.write().await;
    //
    //     let email = Email::parse("test@test.com".to_owned()).unwrap();
    //     let token = generate_auth_token(&email).unwrap();
    //
    //     store
    //         .add_token(token.clone())
    //         .await
    //         .expect("Failed to add token");
    //
    //     let result = store.add_token(token).await;
    //
    //     assert_eq!(
    //         result,
    //         Err(crate::domain::BannedTokenStoreError::TokenAlreadyExists)
    //     )
    // }
}
