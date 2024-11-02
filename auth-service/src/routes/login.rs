use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    domain::{AuthAPIError, Email, Password},
    store::AppState,
    utils::auth::generate_auth_cookie,
};

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
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let LoginRequest {
        email: email_json,
        password: password_json,
    } = request;

    let email = match Email::parse(email_json) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    let password = match Password::parse(password_json) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = state.user_store.read().await;

    if user_store
        .verify_user(&email, &password).await
        .is_err()
    {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    match user_store.get_user(email).await{
        Ok(user) => {
            let response = Json(LoginResponse {
                message: "User Login Successful!".to_string(),
            });

            let auth_cookie = match generate_auth_cookie(&user.email)
                .map_err(|_| AuthAPIError::UnexpectedError)
            {
                Err(e) => return (jar, Err(e)),
                Ok(cookie) => cookie,
            };

            let updated_jar = jar.add(auth_cookie);

            (updated_jar, Ok((StatusCode::OK, response)))
        }
        Err(_) => {
            println!("user not found");
            (jar, Err(AuthAPIError::UnexpectedError))
        }
    }
}
