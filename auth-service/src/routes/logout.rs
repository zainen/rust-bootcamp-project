use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    store::AppState,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    state: State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        None => return (jar, Err(AuthAPIError::MissingToken)),
        Some(cookie) => cookie,
    };

    let token = cookie.value().to_owned();
    match validate_token(&state.banned_tokens_store, &token).await {
        Ok(_) => (),
        Err(e) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    match state.banned_tokens_store.write().await.add_token(token).await {
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
        Ok(_) => {
            let jar = jar.remove(JWT_COOKIE_NAME);
            (jar, Ok(StatusCode::OK))
        }
    }
}
