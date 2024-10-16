use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{domain::User, AppState};

pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let SignupRequest {
        email,
        password,
        requires_2fa,
    } = request;

    let mut user_store = state.user_store.write().await;

    let _result = user_store
        .add_user(User {
            email,
            password,
            requires_2fa,
        })
        .unwrap();

    let response = Json(SignupResponse {
        message: "User Successfully Created".to_owned(),
    });

    (StatusCode::CREATED, response)
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}
