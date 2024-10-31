use auth_service::{domain::Email, utils::auth::generate_auth_cookie};

use super::helpers::TestApp; 

#[tokio::test]
async fn should_return_200_if_valid_token() {
    let app = TestApp::new().await;

    let token = generate_auth_cookie(&Email::parse("test@test.com".to_owned()).expect("Email Failed")).expect("Failed to create cookie");

    let body = serde_json::json!({
        "token": token.value().to_string()
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 200, "Failed for token {:?}", token.to_string());
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "token": "".to_string()
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_401_if_malformed_input() {
    let app = TestApp::new().await;


    let body = serde_json::json!({
        "token": "something invalid".to_string()
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status().as_u16(), 401);
}

