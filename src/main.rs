mod certificates;
mod intercept;
mod proxy;
mod ui;
mod utils;

use anyhow::{Result, Context};
use log::info;

use crate::certificates::ca::CertificateAuthority;
use crate::proxy::server::ProxyServer;
use crate::ui::cli::{parse_cli, Commands};
use crate::utils::logger;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    logger::init()?;

    info!("Starting Ferrum - Web Proxy/Interceptor");

    // Parse command line arguments
    let cli = parse_cli();

    match cli.command {
        Commands::Proxy { addr } => {
            // Initialize Certificate Authority
            let home_dir = dirs::home_dir().context("Failed to get home directory")?;
            let ca_dir = home_dir.join(".ferrum").join("certs");

            let ca = CertificateAuthority::new(
                ca_dir.join("ca.crt"),
                ca_dir.join("ca.key"),
            );

            ca.init()?;

            // Start proxy server
            let server = ProxyServer::new(addr);
            server.start().await?;
        }
    }

    Ok(())
}
