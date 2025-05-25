use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use anyhow::Result;
use hyper::{body::Incoming, Request, Uri};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use hyper_util::client::legacy::connect::HttpConnector;
use http_body_util::Empty;
use bytes::Bytes;
use ferrum::proxy::server::ProxyServer;

#[tokio::test]
async fn test_proxy_server_starts_and_accepts_connections() -> Result<()> {
    // Choose an available port
    let addr: SocketAddr = "127.0.0.1:0".parse()?; // Port 0 means "any available port"

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

    // Try to connect to the server
    let result = timeout(
        Duration::from_secs(1),
        TcpStream::connect(server_addr)
    ).await;

    // Assert that the connection was successful
    assert!(result.is_ok(), "Failed to connect to the proxy server");

    // Clean up
    server_handle.abort();

    Ok(())
}

#[tokio::test]
async fn test_proxy_server_responds_to_requests() -> Result<()> {
    // Choose an available port
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

    // Create a request to the proxy
    let uri = format!("http://{}", server_addr);
    let req = Request::builder()
        .uri(uri)
        .method("GET")
        .body(Empty::<Bytes>::new())?;

    // Send the request
    let resp = client.request(req).await?;

    // Assert that we got a successful response
    assert_eq!(resp.status(), 200, "Expected 200 OK response");

    // Clean up
    server_handle.abort();

    Ok(())
}
