use super::helpers::TestApp; 

#[tokio::test]
async fn logout() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}
