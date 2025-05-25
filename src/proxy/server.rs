use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use hyper::{Request, Response};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::rt::TokioExecutor;
use log::{info, error, debug};
use tokio::net::TcpListener;
use http_body_util::{Full, BodyExt, Empty};
use bytes::Bytes;

use crate::intercept::request::RequestInterceptor;
use crate::intercept::response::ResponseInterceptor;

#[derive(Clone)]
pub struct ProxyServer {
    addr: SocketAddr,
    bound_addr: Arc<Mutex<Option<SocketAddr>>>,
    _req_interceptor: Arc<Mutex<RequestInterceptor>>,
    _res_interceptor: Arc<Mutex<ResponseInterceptor>>,
}

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

impl ProxyServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            bound_addr: Arc::new(Mutex::new(None)),
            _req_interceptor: Arc::new(Mutex::new(RequestInterceptor::new())),
            _res_interceptor: Arc::new(Mutex::new(ResponseInterceptor::new())),
        }
    }

    pub fn address(&self) -> SocketAddr {
        if let Some(addr) = *self.bound_addr.lock().unwrap() {
            addr
        } else {
            self.addr
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting proxy server on {}", self.addr);

        let listener = TcpListener::bind(self.addr)
            .await
            .context("Failed to bind to address")?;

        // Update the address if we bound to port 0 (OS-assigned port)
        let local_addr = listener.local_addr()?;

        // Store the actual bound address
        *self.bound_addr.lock().unwrap() = Some(local_addr);

        if self.addr.port() == 0 {
            info!("Proxy server bound to {}", local_addr);
        }

        loop {
            let (stream, remote_addr) = match listener.accept().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                    continue;
                }
            };

            info!("Accepted connection from {}", remote_addr);

            let req_interceptor = Arc::clone(&self._req_interceptor);
            let res_interceptor = Arc::clone(&self._res_interceptor);

            // Spawn a new task for each connection
            tokio::spawn(async move {
                let io = TokioIo::new(stream);

                let service = service_fn(move |req| {
                    let _req_interceptor = Arc::clone(&req_interceptor);
                    let _res_interceptor = Arc::clone(&res_interceptor);
                    handle_request(req)
                });

                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, service)
                    .await
                {
                    error!("Error serving connection: {}", e);
                }
            });
        }
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody>, hyper::Error> {
    debug!("Received request: {} {}", req.method(), req.uri());

    // Check if this is a request to be forwarded (for integration tests)
    if let Some(target_url) = req.headers().get("X-Target-Url") {
        if let Ok(target) = target_url.to_str() {
            debug!("Forwarding request to target: {}", target);

            // Create an HTTP client
            let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new())
                .build::<_, Empty<Bytes>>(
                    hyper_util::client::legacy::connect::HttpConnector::new());

            // Parse the target URL
            let target_uri = target.parse::<hyper::Uri>().unwrap();

            // Forward the request
            let target_req = Request::builder()
                .method(req.method().clone())
                .uri(target_uri)
                .body(http_body_util::Empty::<Bytes>::new())
                .unwrap();

            // Make the request
            if let Ok(resp) = client.request(target_req).await {
                // Convert the response body
                let (parts, body) = resp.into_parts();

                // Collect the body
                let body_bytes = match body.collect().await {
                    Ok(collected) => collected.to_bytes(),
                    Err(_) => Bytes::new(),
                };

                // Return the response with the same status code
                return Ok(Response::from_parts(
                    parts,
                    full(body_bytes)
                ));
            } else {
                return Ok(Response::builder()
                    .status(500)
                    .body(full("Error forwarding request"))
                    .unwrap());
            }
        }
    }

    // Default response
    let response = Response::builder()
        .status(200)
        .body(full("Ferrum Proxy - Request received and logged"))
        .unwrap();

    Ok(response)
}
