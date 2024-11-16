use crate::helpers::get_random_email;

use super::helpers::TestApp;
use auth_service::{
    domain::{Email, ErrorResponse},
    utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME},
};
use reqwest::Url;
use secrecy::Secret;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let mut app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(
        response.status().as_u16(),
        400,
        "failed for {:?}",
        app.cookie_jar
    );
    app.clean_up().await
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let mut app = TestApp::new().await;

    let cookie = generate_auth_cookie(
        &Email::parse(Secret::new("test@test.com".to_owned())).expect("Failed to parse email"),
    )
    .expect("Email failed");
    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app
        .post_verify_token(&serde_json::json!({"token": &cookie.value().to_string()}))
        .await;

    assert_eq!(
        response.status().as_u16(),
        200,
        "{:?}",
        cookie.value().to_string()
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
    app.clean_up().await
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice() {
    let mut app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(auth_cookie.value().is_empty());

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Missing Token".to_owned()
    );
    app.clean_up().await
}
