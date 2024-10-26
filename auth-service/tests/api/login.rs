use crate::helpers::get_random_email;

use super::helpers::TestApp; 

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    
    let test_case = [
        serde_json::json!({
            "email": get_random_email(),
        }),
        serde_json::json!({
            "password": get_random_email(),
        })
    ];

    for test_case in test_case.iter() {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;


    let email = get_random_email();
    let password = "bad".to_owned();
    
    let test_case = [
        serde_json::json!({
            "email": email,
            "password": password
        }),
    ];

    for test_case in test_case.iter() {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let email = "email@email.com".to_owned();

    let resp = app.post_signup(&serde_json::json!({
        "email": email.clone(),
        "password": "password123!!@#",
        "requires2FA": true,
    })).await;

        assert_eq!(resp.status().as_u16(), 201,);

    let test_case = [
        serde_json::json!({
            "email": email.clone(),
            "password": "soemthingwrong".to_owned()
        }),
    ];

    for test_case in test_case.iter() {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input: {:?}",
            test_case
        );
    }
}
