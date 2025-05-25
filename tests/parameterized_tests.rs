use hyper::{Request, Method, Uri, Response, StatusCode};
use http_body_util::{Full, BodyExt, Empty};
use bytes::Bytes;
use anyhow::Result;
use std::net::SocketAddr;
use rstest::*;
use tokio::time::Duration;

use ferrum::proxy::server::ProxyServer;
use ferrum::intercept::request::RequestInterceptor;
use ferrum::intercept::response::ResponseInterceptor;

mod test_utils;
use test_utils::{init_test_logging, get_test_addr};

struct RequestFixture {
    method: Method,
    uri: Uri,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestFixture {
    fn new(method: Method, uri: &str) -> Self {
        Self {
            method,
            uri: uri.parse().unwrap(),
            headers: Vec::new(),
            body: None,
        }
    }

    fn with_header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    fn with_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    fn build(&self) -> Request<Empty<Bytes>> {
        let mut builder = Request::builder()
            .method(self.method.clone())
            .uri(self.uri.clone());

        for (name, value) in &self.headers {
            builder = builder.header(name, value);
        }

        // Create an empty body instead of using Incoming::default()
        builder.body(Empty::<Bytes>::new()).unwrap()
    }
}

#[fixture]
fn get_request() -> RequestFixture {
    RequestFixture::new(Method::GET, "http://example.com/test")
}

#[fixture]
fn post_request() -> RequestFixture {
    RequestFixture::new(Method::POST, "http://example.com/test")
        .with_body("test=data")
}

#[fixture]
fn proxy_server() -> ProxyServer {
    ProxyServer::new(get_test_addr())
}

#[rstest]
#[tokio::test]
async fn test_proxy_handles_various_methods(
    #[values(Method::GET, Method::POST, Method::PUT, Method::DELETE)] method: Method,
) -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a request with the specified method
    let request = RequestFixture::new(method.clone(), "http://example.com/test").build();

    // Create a request interceptor
    let interceptor = RequestInterceptor::new();

    // Process the request
    let result = interceptor.intercept(request).await?;

    // Verify that the method is preserved
    assert_eq!(result.method(), &method, "Method should be unchanged");

    Ok(())
}

#[rstest]
#[tokio::test]
async fn test_response_interceptor_handles_various_statuses(
    #[values(StatusCode::OK, StatusCode::NOT_FOUND, StatusCode::INTERNAL_SERVER_ERROR)] status: StatusCode,
) -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a response with the specified status
    let response = Response::builder()
        .status(status)
        .body(Full::new(Bytes::from("Test body")).map_err(|_| panic!("This never happens")).boxed())?;

    // Create a response interceptor
    let interceptor = ResponseInterceptor::new();

    // Process the response
    let result = interceptor.intercept(response).await?;

    // Verify that the status is preserved
    assert_eq!(result.status(), status, "Status should be unchanged");

    Ok(())
}
