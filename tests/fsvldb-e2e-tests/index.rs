use exn::Exn;
use exn::ResultExt;
use reqwest::StatusCode;

use crate::conftest::{TestApp, TestError};

#[tokio::test]
async fn test_index_html_no_input_returns_200() -> Result<(), Exn<TestError>> {
    let make_error = || TestError;
    // Arrange
    let app = TestApp::spawn_app().await.or_raise(make_error)?;
    let client = reqwest::Client::new();

    // Act
    let response = client.get(app.index()).send().await.or_raise(make_error)?;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    let html: String = response.text().await.or_raise(make_error)?;

    insta::assert_snapshot!(html);

    Ok(())
}
