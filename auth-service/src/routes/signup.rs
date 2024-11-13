use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use secrecy::Secret;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{AuthAPIError, Email, Password, User},
    AppState,
};

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    state: State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let SignupRequest {
        email,
        password,
        requires_2fa,
    } = request;
    let email_parsed = match Email::parse(email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    let password_parsed = match Password::parse(password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let user = User::new(email_parsed, password_parsed, requires_2fa);

    let mut user_store = state.user_store.write().await;

    match user_store.get_user(&user.email.clone()).await {
        Err(_) => {}
        Ok(_) => return Err(AuthAPIError::UserAlreadyExists),
    };

    let result = user_store.add_user(user).await;

    match result {
        Err(e) => return Err(AuthAPIError::UnexpectedError(e.into())),
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User Created Successfully!".to_string(),
            });

            Ok((StatusCode::CREATED, response))
        }
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: Secret<String>,
    pub password: Secret<String>,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}
