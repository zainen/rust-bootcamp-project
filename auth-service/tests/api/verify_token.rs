use auth_service::utils::constants::JWT_COOKIE_NAME;

use super::helpers::TestApp;

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let mut app = TestApp::new().await;

    let response = app
        .post_signup(&serde_json::json!({
            "email": "test@test.com",
            "password": "password123",
            "requires2FA": false
        }))
        .await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app
        .post_login(&serde_json::json!({
            "email": "test@test.com",
            "password": "password123"
        }))
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let token = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("Failed to find cookie");
    assert!(!token.value().is_empty());

    let body = serde_json::json!({
        "token": token.value()
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(
        response.status().as_u16(),
        200,
        "Failed for token {:?}",
        token
    );
    app.clean_up().await
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let body = serde_json::json!({
        "token": "".to_string()
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 422);
    app.clean_up().await
}

#[tokio::test]
async fn should_return_401_if_malformed_input() {
    let mut app = TestApp::new().await;

    let body = serde_json::json!({
        "token": "something invalid".to_string()
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await
}
