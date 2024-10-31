use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{domain::{AuthAPIError, Email, Password}, store::AppState, utils::auth::generate_auth_cookie};

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
    Json(request): Json<LoginRequest>
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let LoginRequest {
        email: email_json,
        password: password_json
    } = request;

    let email = match Email::parse(email_json) {
        Ok(email) => email,
        Err(e) => return (jar, Err(e))
    };
    let password = match Password::parse(password_json) {
        Ok(password) => password,
        Err(err) => return (jar, Err(err))
    };

    let user_store = state.user_store.read().await;

    match user_store.get_user(email) {
        Ok(user) => {
            if user.password != password {
                return (jar, Err(AuthAPIError::IncorrectCredentials))
            }
            let response = Json(LoginResponse {
                message: "User Created Successfully!".to_string()
            });

            let auth_cookie = generate_auth_cookie(&user.email).map_err(|_| AuthAPIError::UnexpectedError);
            if let Err(e) = auth_cookie {
                return (jar, Err(e))
            }

            let updated_jar = jar.add(auth_cookie.unwrap());


            (updated_jar, Ok((StatusCode::OK, response)))
        },
        Err(_) => {
            println!("user not found");
            (jar, Err(AuthAPIError::UnexpectedError))
        }
    }

}
