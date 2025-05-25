use std::path::{Path, PathBuf};
use tempfile::TempDir;
use anyhow::Result;
use ferrum::certificates::ca::CertificateAuthority;
use crate::test_utils::init_test_logging;

#[tokio::test]
async fn test_ca_initialization() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a temporary directory for the test
    let temp_dir = TempDir::new()?;
    let ca_cert_path = temp_dir.path().join("ca.crt");
    let ca_key_path = temp_dir.path().join("ca.key");

    // Create a new certificate authority
    let ca = CertificateAuthority::new(ca_cert_path.clone(), ca_key_path.clone());

    // Initialize the CA
    ca.init()?;

    // Assert that the certificate and key files were created
    assert!(ca_cert_path.exists(), "CA certificate file was not created");
    assert!(ca_key_path.exists(), "CA key file was not created");

    // Test idempotence - should not error when called again
    ca.init()?;

    // Test the getters
    assert_eq!(ca.get_ca_cert_path(), &ca_cert_path);
    assert_eq!(ca.get_ca_key_path(), &ca_key_path);

    Ok(())
}

#[tokio::test]
async fn test_generate_cert_for_domain() -> Result<()> {
    // Initialize test logging
    init_test_logging();

    // Create a temporary directory for the test
    let temp_dir = TempDir::new()?;
    let ca_cert_path = temp_dir.path().join("ca.crt");
    let ca_key_path = temp_dir.path().join("ca.key");

    // Create and initialize a new certificate authority
    let ca = CertificateAuthority::new(ca_cert_path, ca_key_path);
    ca.init()?;

    // Generate a certificate for a domain
    let domain = "example.com";
    let (cert, key) = ca.generate_cert_for_domain(domain)?;

    // In our simple implementation, we're just checking that we got some bytes back
    assert!(!cert.is_empty(), "Certificate should not be empty");
    assert!(!key.is_empty(), "Key should not be empty");

    // Check that the certificate contains the domain name
    let cert_str = String::from_utf8(cert)?;
    assert!(cert_str.contains(domain), "Certificate should contain the domain name");

    Ok(())
}
