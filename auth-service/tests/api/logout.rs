use super::helpers::TestApp;
use auth_service::{
    domain::{Email, ErrorResponse},
    utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME},
};
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(
        response.status().as_u16(),
        400,
        "failed for {:?}",
        app.cookie_jar
    );
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;

    let cookie = generate_auth_cookie(
        &Email::parse("test@test.com".to_owned()).expect("Failed to parse email"),
    )
    .expect("Email failed");
    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice() {
    let app = TestApp::new().await;

    let cookie = generate_auth_cookie(
        &Email::parse("test@test.com".to_owned()).expect("Failed to parse email"),
    )
    .expect("Email failed");
    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 400);

}
