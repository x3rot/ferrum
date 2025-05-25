use hyper::{Response, StatusCode};
use anyhow::Result;
use bytes::Bytes;
use http_body_util::{Full, BodyExt};
use ferrum::intercept::response::ResponseInterceptor;
use crate::test_utils::init_test_logging;

#[tokio::test]
async fn test_response_interceptor_disable_enable() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a new response interceptor
    let mut interceptor = ResponseInterceptor::new();

    // By default, it should be enabled
    assert!(interceptor.is_enabled(), "Interceptor should be enabled by default");

    // Disable it
    interceptor.disable();
    assert!(!interceptor.is_enabled(), "Interceptor should be disabled after calling disable()");

    // Enable it again
    interceptor.enable();
    assert!(interceptor.is_enabled(), "Interceptor should be enabled after calling enable()");

    Ok(())
}

#[tokio::test]
async fn test_response_interceptor_passthrough_when_disabled() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a test response
    let status = StatusCode::OK;
    let body = "Test response body";
    let body_bytes = Bytes::from(body);

    let response = Response::builder()
        .status(status)
        .body(Full::new(body_bytes.clone()).map_err(|_| panic!("This never happens")).boxed())?;

    // Create and disable a response interceptor
    let mut interceptor = ResponseInterceptor::new();
    interceptor.disable();

    // Process the response
    let result = interceptor.intercept(response).await?;

    // Since the interceptor is disabled, the response should pass through unchanged
    assert_eq!(result.status(), status, "Status should be unchanged");

    Ok(())
}

#[tokio::test]
async fn test_response_interceptor_processes_when_enabled() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a test response
    let status = StatusCode::OK;
    let body = "Test response body";
    let body_bytes = Bytes::from(body);

    let response = Response::builder()
        .status(status)
        .body(Full::new(body_bytes.clone()).map_err(|_| panic!("This never happens")).boxed())?;

    // Create an interceptor (enabled by default)
    let interceptor = ResponseInterceptor::new();

    // Process the response
    let result = interceptor.intercept(response).await?;

    // In our current implementation, the interceptor just logs and passes through
    // In a real test, we would verify that any transformations were applied
    assert_eq!(result.status(), status, "Status should be unchanged");

    Ok(())
}
