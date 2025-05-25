use clap::{Parser, Subcommand};
use std::net::SocketAddr;

#[derive(Parser)]
#[command(author, version, about = "A web proxy/interceptor tool built in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the proxy server
    Proxy {
        /// Address to bind the proxy to
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: SocketAddr,
    },
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
