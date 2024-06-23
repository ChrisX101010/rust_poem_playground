use poem::{error::NotFoundError, handler, http::StatusCode, test::TestClient, Result};

#[handler]
fn return_str() -> &'static str {
    "hello"
}

#[handler]
fn return_err() -> Result<&'static str, NotFoundError> {
    Err(NotFoundError)
}

#[tokio::test]
async fn test_endpoints() {
    let client = TestClient::new(return_str);
    let resp = client.get("/").send().await;
    resp.assert_status_is_ok();
    resp.assert_text("hello").await.unwrap();

    let client = TestClient::new(return_err);
    let resp = client.get("/").send().await;
    resp.assert_status(StatusCode::NOT_FOUND);
}
