use poem::test::TestClient;
use tms_min::baseline;

#[tokio::test]
async fn test_baseline() {
    // Create a test client with the handler as the endpoint
    let cli = TestClient::new(baseline);

    // Send a GET request to the handler
    let resp = cli.get("/").send().await;

    // Validate
    resp.assert_status_is_ok();
    resp.assert_text("Server is running");
}
