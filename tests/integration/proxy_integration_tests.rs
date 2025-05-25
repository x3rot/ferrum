use std::net::SocketAddr;
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use anyhow::Result;
use hyper::{body::Incoming, Request, Uri};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use hyper_util::client::legacy::connect::HttpConnector;
use http_body_util::{Empty, BodyExt};
use bytes::Bytes;
use httpmock::MockServer;
use ferrum::proxy::server::ProxyServer;

#[tokio::test]
async fn test_proxy_forwards_requests() -> Result<()> {
    // Start a mock target server
    let mock_server = MockServer::start();

    // Create a mock for a GET request
    let mock = mock_server.mock(|when, then| {
        when.method("GET").path("/test");
        then.status(200).body("Hello from target server");
    });

    // Choose an available port for the proxy
    let addr: SocketAddr = "127.0.0.1:0".parse()?;

    // Create and start the proxy server in a separate task
    let server = ProxyServer::new(addr);
    let server_clone = server.clone();

    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            eprintln!("Server error: {}", e);
        }
    });

    // Give the server a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Get the actual bound address after starting
    let server_addr = server_clone.address();

    // Create an HTTP client
    let client = Client::builder(TokioExecutor::new())
        .build::<_, Empty<Bytes>>(HttpConnector::new());

    // Create a request through the proxy to the mock server
    // Note: In a real implementation, the proxy would need to support connecting to external URLs
    let target_url = format!("http://{}/test", mock_server.address());
    let uri = format!("http://{}", server_addr);

    let req = Request::builder()
        .uri(uri)
        .method("GET")
        .header("Host", mock_server.address().to_string())
        .header("X-Target-Url", target_url)  // Custom header to tell the proxy where to forward
        .body(Empty::<Bytes>::new())?;

    // Send the request
    let resp = client.request(req).await?;
    let status = resp.status();
    let body_bytes = resp.collect().await?.to_bytes();
    let body_str = String::from_utf8(body_bytes.to_vec())?;

    // Assert that the mock was called
    mock.assert();

    // Assert that we got the expected response
    assert_eq!(status, 200, "Expected 200 OK response");
    assert_eq!(body_str, "Hello from target server", "Unexpected response body");

    // Clean up
    server_handle.abort();

    Ok(())
}

#[tokio::test]
async fn test_proxy_handles_different_http_methods() -> Result<()> {
    // Start a mock target server
    let mock_server = MockServer::start();

    // Create mocks for different HTTP methods
    let get_mock = mock_server.mock(|when, then| {
        when.method("GET").path("/test");
        then.status(200).body("GET response");
    });

    let post_mock = mock_server.mock(|when, then| {
        when.method("POST").path("/test");
        then.status(201).body("POST response");
    });

    let put_mock = mock_server.mock(|when, then| {
        when.method("PUT").path("/test");
        then.status(200).body("PUT response");
    });

    // Choose an available port for the proxy
    let addr: SocketAddr = "127.0.0.1:0".parse()?;

    // Create and start the proxy server in a separate task
    let server = ProxyServer::new(addr);
    let server_clone = server.clone();

    let server_handle = tokio::spawn(async move {
        if let Err(e) = server.start().await {
            eprintln!("Server error: {}", e);
        }
    });

    // Give the server a moment to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Get the actual bound address after starting
    let server_addr = server_clone.address();

    // Create an HTTP client
    let client = Client::builder(TokioExecutor::new())
        .build::<_, Empty<Bytes>>(HttpConnector::new());

    // Test GET request
    let target_url = format!("http://{}/test", mock_server.address());
    let uri = format!("http://{}", server_addr);

    let get_req = Request::builder()
        .uri(uri.clone())
        .method("GET")
        .header("Host", mock_server.address().to_string())
        .header("X-Target-Url", target_url.clone())
        .body(Empty::<Bytes>::new())?;

    let get_resp = client.request(get_req).await?;
    assert_eq!(get_resp.status(), 200, "Expected 200 OK for GET");
    get_mock.assert();

    // Test POST request
    let post_req = Request::builder()
        .uri(uri.clone())
        .method("POST")
        .header("Host", mock_server.address().to_string())
        .header("X-Target-Url", target_url.clone())
        .body(Empty::<Bytes>::new())?;

    let post_resp = client.request(post_req).await?;
    assert_eq!(post_resp.status(), 201, "Expected 201 Created for POST");
    post_mock.assert();

    // Test PUT request
    let put_req = Request::builder()
        .uri(uri)
        .method("PUT")
        .header("Host", mock_server.address().to_string())
        .header("X-Target-Url", target_url)
        .body(Empty::<Bytes>::new())?;

    let put_resp = client.request(put_req).await?;
    assert_eq!(put_resp.status(), 200, "Expected 200 OK for PUT");
    put_mock.assert();

    // Clean up
    server_handle.abort();

    Ok(())
}
