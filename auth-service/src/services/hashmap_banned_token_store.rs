use std::collections::HashMap;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashmapBannedTokenStore {
    pub tokens: HashMap<String, String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashmapBannedTokenStore {
    fn add_token(&mut self, token: String) -> Result<(), crate::domain::BannedTokenStoreError> {
        if token.is_empty() {
            return Err(BannedTokenStoreError::TokenNotFound);
        }
        // if let Err(e) = validate_token(&token).await {

        // }
        if let Some(_) = self.tokens.get(&token) {
            return Err(BannedTokenStoreError::TokenAlreadyExists);
        }
        self.tokens.insert(token.clone(), token);
        Ok(())
    }

    fn verify_token_exists(&self, token: &str) -> Result<(), crate::domain::BannedTokenStoreError> {
        if token.is_empty() {
            return Err(BannedTokenStoreError::TokenNotFound);
        }

        match self.tokens.get(token) {
            None => return Err(BannedTokenStoreError::TokenNotFound),
            Some(_) => return Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{domain::Email, utils::auth::generate_auth_cookie};

    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashmapBannedTokenStore {
            tokens: HashMap::new(),
        };

        let token =
            generate_auth_cookie(&Email::parse("test@test.com".to_owned()).expect("Email failed"))
                .expect("Token gen failed");

        let result = store
            .add_token(token.value().to_string())
            .expect("Token failed");

        assert_eq!(result, ())
    }

    #[tokio::test]
    async fn test_add_no_token() {
        let mut store = HashmapBannedTokenStore {
            tokens: HashMap::new(),
        };

        let token =
            generate_auth_cookie(&Email::parse("test@test.com".to_owned()).expect("Email failed"))
                .expect("Token gen failed");

        let result = store.add_token("".to_string());

        assert_eq!(result, Err(BannedTokenStoreError::TokenNotFound))
    }

    #[tokio::test]
    async fn test_add_token_already_exists() {
        let mut store = HashmapBannedTokenStore {
            tokens: HashMap::new(),
        };

        let token =
            generate_auth_cookie(&Email::parse("test@test.com".to_owned()).expect("Email failed"))
                .expect("Token gen failed");

        let result = store
            .add_token(token.value().to_string())
            .expect("Token failed");

        assert_eq!(result, ());

        let result = store.add_token(token.value().to_string());

        assert_eq!(result, Err(BannedTokenStoreError::TokenAlreadyExists))
    }
}