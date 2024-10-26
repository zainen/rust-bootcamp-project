use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{domain::{AuthAPIError, Email, Password}, store::AppState};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    state: State<AppState>, 
    Json(request): Json<LoginRequest>
) -> Result<impl IntoResponse, AuthAPIError> {
    let LoginRequest {
        email: email_json,
        password: password_json
    } = request;

    let email = match Email::parse(email_json) {
        Ok(email) => email,
        Err(e) => return Err(e)
    };
    let password = match Password::parse(password_json) {
        Ok(password) => password,
        Err(err) => return Err(err)
    };

    let user_store = state.user_store.read().await;

    match user_store.get_user(email) {
        Ok(user) => {
            if user.password != password {
                return Err(AuthAPIError::IncorrectCredentials)
            }
            let response = Json(LoginResponse {
                message: "User Created Successfully!".to_string()
            });

            Ok((StatusCode::CREATED, response))
        },
        Err(_) => {
            println!("user not found");
            Err(AuthAPIError::UnexpectedError)
        }
    }

}
