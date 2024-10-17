use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{domain::{User, AuthAPIError}, AppState};

pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let SignupRequest {
        email,
        password,
        requires_2fa,
    } = request;

    if !email.clone().contains("@") || password.clone().len() < 8  {
        println!("here");
        return Err(AuthAPIError::InvalidCredentials)
    }

    let user = User::new(email, password, requires_2fa);

    let mut user_store = state.user_store.write().await;

    match user_store.get_user(&user.email) {
        Err(_) => {},
        Ok(_) => {
            return Err(AuthAPIError::UserAlreadyExists)
        }
    };



    let result = user_store
        .add_user(user);

    match result {
        Err(_) => {
            return Err(AuthAPIError::UnexpectedError)
        },
        Ok(_) => {

            
            let response = Json(SignupResponse {
                message: "User Created Successfully!".to_string()
            });
            
            Ok((StatusCode::CREATED, response))
        }
    }

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
