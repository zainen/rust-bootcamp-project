use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate:: {
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match  jar.get(JWT_COOKIE_NAME) {
        None => return (jar, Err(AuthAPIError::MissingToken)),
        Some(cookie) => cookie,
    };

    let token = cookie.value().to_owned();
    if token.is_empty() {
        return (jar, Err(AuthAPIError::MissingToken))
    }

    match validate_token(&token).await {
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
        Ok(_) => {
            let jar = jar.clone().remove(JWT_COOKIE_NAME);
            (jar, Ok(StatusCode::OK))
        }
    }
    
}
