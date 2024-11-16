use auth_service::{
    domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME,
};
use secrecy::Secret;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::get_random_email;

use super::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let test_values = [
        serde_json::json!({
            "email": "test@test.com",
            "password": "12312323",
        }),
        serde_json::json!({
            "email": "test@test.com",
            "2FACode": "12312323",
        }),
    ];

    for test in test_values {
        let response = app.post_verify_2fa(&test).await;
        assert_eq!(response.status().as_u16(), 422);
    }
    app.clean_up().await
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;

    let test_values = [
        serde_json::json!({
            "email": "test@test.com",
            "loginAttemptId": "12312323",
            "2FACode": "12092381"
        }),
        serde_json::json!({
            "email": "test@test.com",
            "loginAttemptId": "12312323",
            "2FACode": "098231928309182309",
        }),
    ];

    for test in test_values {
        let response = app.post_verify_2fa(&test).await;
        assert_eq!(response.status().as_u16(), 400);
    }
    app.clean_up().await
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(2)
        .mount(&app.email_server)
        .await;

    // First login call

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(Secret::new(random_email.clone())).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    // Second login call

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // 2FA attempt with old login_attempt_id and code

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 401);

    app.clean_up().await
}

#[tokio::test]
async fn should_return_200() {
    let mut app = TestApp::new().await;
    let email = Email::parse(Secret::new(get_random_email())).unwrap();

    let _response = app
        .post_signup(&serde_json::json!({
            "email": email.as_ref(),
            "password": "password123",
            "requires2FA": true,
        }))
        .await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = app
        .post_login(&serde_json::json!({
            "email": email.as_ref(),
            "password": "password123",
        }))
        .await;

    assert_eq!(response.status().as_u16(), 206);

    let (attempt_id, code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .expect("Failed to get code for email");

    assert!(!code.clone().as_ref().is_empty());
    assert!(!attempt_id.clone().as_ref().is_empty());
    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": email.as_ref(),
            "loginAttemptId": attempt_id.clone().as_ref(),
            "2FACode": code.as_ref()
        }))
        .await;

    assert_eq!(
        response.status().as_u16(),
        200,
        "2fa: {}, attempt: {}",
        code.as_ref(),
        attempt_id.as_ref()
    );

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
    app.clean_up().await
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let mut app = TestApp::new().await;
    let email = Email::parse(Secret::new(get_random_email())).unwrap();

    let _response = app
        .post_signup(&serde_json::json!({
            "email": email.as_ref(),
            "password": "password123",
            "requires2FA": true,
        }))
        .await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = app
        .post_login(&serde_json::json!({
            "email": email.as_ref(),
            "password": "password123",
        }))
        .await;

    assert_eq!(response.status().as_u16(), 206);

    let (attempt_id, code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .expect("Failed to get code for email");

    assert!(!code.clone().as_ref().is_empty());
    assert!(!attempt_id.clone().as_ref().is_empty());
    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": email.as_ref(),
            "loginAttemptId": attempt_id.clone().as_ref(),
            "2FACode": code.as_ref()
        }))
        .await;

    assert_eq!(
        response.status().as_u16(),
        200,
        "2fa: {}, attempt: {}",
        code.clone().as_ref(),
        attempt_id.clone().as_ref()
    );

    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": email.as_ref(),
            "loginAttemptId": attempt_id.clone().as_ref(),
            "2FACode": code.as_ref()
        }))
        .await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "2fa: {}, attempt: {}",
        code.clone().as_ref(),
        attempt_id.clone().as_ref()
    );
    app.clean_up().await
}
