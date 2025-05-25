// Expose modules for testing
pub mod certificates;
pub mod intercept;
pub mod proxy;
pub mod ui;
pub mod utils;

// Re-export the core types for convenient importing
pub use crate::certificates::ca::CertificateAuthority;
pub use crate::proxy::server::ProxyServer;
pub use crate::intercept::request::RequestInterceptor;
pub use crate::intercept::response::ResponseInterceptor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // A simple test to verify the library is loadable
        assert!(true);
    }
}
