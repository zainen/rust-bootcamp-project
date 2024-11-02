use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{store::AppState, utils::auth::validate_token};

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

// #[derive(Serialize)]
// pub struct VerifyTokenResponse {
//     pub message: String
// }

pub async fn verify_token(
    state: State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
    let VerifyTokenRequest { token } = request;

    if token.is_empty() {
        return StatusCode::UNPROCESSABLE_ENTITY;
    }

    let banned_store = &state.banned_tokens_store;

    match validate_token(banned_store, &token).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}
