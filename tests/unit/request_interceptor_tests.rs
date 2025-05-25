use hyper::{Request, Uri, Method, body::Incoming};
use anyhow::Result;
use bytes::Bytes;
use ferrum::intercept::request::RequestInterceptor;
use crate::test_utils::init_test_logging;
use http_body_util::Empty;

#[tokio::test]
async fn test_request_interceptor_disable_enable() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a new request interceptor
    let mut interceptor = RequestInterceptor::new();

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
async fn test_request_interceptor_passthrough_when_disabled() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a test request
    let uri = Uri::from_static("http://example.com");
    let method = Method::GET;
    let request = Request::builder()
        .uri(uri.clone())
        .method(method.clone())
        .body(Empty::<Bytes>::new())?;

    // Create and disable a request interceptor
    let mut interceptor = RequestInterceptor::new();
    interceptor.disable();

    // Process the request
    let result = interceptor.intercept(request).await?;

    // Since the interceptor is disabled, the request should pass through unchanged
    assert_eq!(result.uri(), &uri, "URI should be unchanged");
    assert_eq!(result.method(), &method, "Method should be unchanged");

    Ok(())
}

#[tokio::test]
async fn test_request_interceptor_processes_when_enabled() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a test request
    let uri = Uri::from_static("http://example.com");
    let method = Method::GET;
    let request = Request::builder()
        .uri(uri.clone())
        .method(method.clone())
        .body(Empty::<Bytes>::new())?;

    // Create an interceptor (enabled by default)
    let interceptor = RequestInterceptor::new();

    // Process the request
    let result = interceptor.intercept(request).await?;

    // In our current implementation, the interceptor just logs and passes through
    // In a real test, we would verify that any transformations were applied
    assert_eq!(result.uri(), &uri, "URI should be unchanged");
    assert_eq!(result.method(), &method, "Method should be unchanged");

    Ok(())
}
