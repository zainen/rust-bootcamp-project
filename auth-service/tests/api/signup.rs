use super::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

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
}

#[tokio::test]
async fn should_return_201_if_proper_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_case = [
        serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true,
        }),
        serde_json::json!({
        "email": get_random_email(),
        "password": "password123",
        "requires2FA": false,
        }),
    ];

    for test_case in test_case.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            201,
        );
    }
}
