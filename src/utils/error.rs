use std::fmt;

#[derive(Debug)]
pub enum FerrumError {
    ProxyError(String),
    CertificateError(String),
    NetworkError(String),
}

impl fmt::Display for FerrumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProxyError(msg) => write!(f, "Proxy error: {}", msg),
            Self::CertificateError(msg) => write!(f, "Certificate error: {}", msg),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for FerrumError {}
