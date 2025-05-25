//! Test configuration and utilities

use std::net::SocketAddr;
use std::sync::Once;
use log::LevelFilter;
use env_logger::Builder;
use std::io::Write;

// Initialize the logger once for all tests
static INIT: Once = Once::new();

/// Initialize test logging
pub fn init_test_logging() {
    INIT.call_once(|| {
        let mut builder = Builder::new();
        builder
            .filter_level(LevelFilter::Debug)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] {} - {}",
                    record.level(),
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.args()
                )
            })
            .init();
    });
}

/// Get a dynamic test port
pub fn get_test_port() -> u16 {
    // Port 0 will make the OS assign an available port
    let socket = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    let addr = socket.local_addr().expect("Failed to get local address");
    addr.port()
}

/// Get a dynamic test address
pub fn get_test_addr() -> SocketAddr {
    format!("127.0.0.1:{}", get_test_port()).parse().unwrap()
}
