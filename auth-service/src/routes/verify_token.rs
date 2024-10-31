use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{store::AppState, utils::auth::validate_token};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String
}

// #[derive(Serialize)]
// pub struct VerifyTokenResponse {
//     pub message: String
// }

pub async fn verify_token( Json(request): Json<VerifyTokenRequest>) -> impl IntoResponse {
    let VerifyTokenRequest {token} = request;

    if token.is_empty() {
        return StatusCode::UNPROCESSABLE_ENTITY
    }

    match validate_token(&token).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED
    }


}
