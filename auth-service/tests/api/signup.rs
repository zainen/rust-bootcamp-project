use super::helpers::{get_random_email, TestApp};
use auth_service::domain::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let test_case = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
    ];

    for test_case in test_case.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed  for input: {:?}",
            test_case
        );
    }
    app.clean_up().await
}

#[tokio::test]
async fn should_return_201_if_proper_input() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email();

    let test_case = [
        serde_json::json!({
        "email": random_email,
        "password": "password123!!@#",
        "requires2FA": true,
        }),
        serde_json::json!({
        "email": get_random_email(),
        "password": "password123@#!",
        "requires2FA": false,
        }),
    ];

    for test_case in test_case.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(response.status().as_u16(), 201,);
    }
    app.clean_up().await
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;

    let test_case = [
        serde_json::json!({
        "email": "email@email.com".to_owned(),
        "password": "less".to_owned(),
        "requires2FA": true,
        }),
        serde_json::json!({
        "email": "email.com".to_owned(),
        "password": "asdfghkld".to_owned(),
        "requires2FA": true,
        }),
    ];

    for i in test_case.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}:{:?}",
            i,
            response
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
    app.clean_up().await
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let mut app = TestApp::new().await;

    let user_1 = serde_json::json!({
    "email": "something@example.com".to_owned(),
    "password": "password1!@#S".to_owned(),
    "requires2FA": true,
    });

    app.post_signup(&user_1).await;

    let response = app.post_signup(&user_1).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
    app.clean_up().await
}
