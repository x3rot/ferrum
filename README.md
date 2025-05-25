`# Ferrum - HTTP Proxy for Debugging and Testing

Ferrum is a lightweight HTTP proxy written in Rust, designed for debugging, inspecting, and potentially modifying HTTP traffic. It's particularly useful for developers who need to understand the communication between web applications and APIs.

## Features

- **HTTP Proxy**: Intercepts and forwards HTTP requests and responses
- **Certificate Authority**: Generate and manage certificates for HTTPS inspection
- **Traffic Interception**: View and modify requests and responses on the fly
- **Extensible Architecture**: Modular design for easy addition of new features

## Project Structure

The project is organized into several modules:

- **certificates**: Handles the creation and management of a certificate authority for HTTPS inspection
- **intercept**: Contains interceptors for requests and responses that can view or modify HTTP traffic
- **proxy**: Core proxy server implementation that handles connections and forwarding
- **ui**: Command-line and terminal UI interfaces for interacting with the proxy
- **utils**: Utility functions for error handling, logging, etc.

## Installation

### Prerequisites

- Rust and Cargo (install via [rustup](https://rustup.rs/))

### Building from Source

Clone the repository and build with Cargo:

```bash
git clone https://github.com/username/ferrum.git
cd ferrum
cargo build --release
```

The compiled binary will be available at `target/release/ferrum`.

## Usage

### Basic Proxy

Start Ferrum as a simple HTTP proxy:

```bash
./ferrum --port 8080
```

This will start the proxy server listening on port 8080. Configure your browser or application to use this proxy.

### With Request/Response Interception

To enable request and response interception for viewing or modifying traffic:

```bash
./ferrum --port 8080 --intercept
```

### HTTPS Inspection

To enable HTTPS inspection (requires generating and installing a CA certificate):

```bash
./ferrum --port 8080 --https-inspect
```

On first run, this will generate a CA certificate. You will need to install this certificate in your browser or operating system to avoid security warnings.

## Development

### Running Tests

Run the test suite with:

```bash
cargo test
```

The project includes unit tests, parameterized tests, and integration tests to verify functionality.

### Project Structure

- **src/**: Contains the source code organized by module
- **tests/**: Contains test files for validating functionality
  - **unit/**: Unit tests for individual components
  - **integration/**: Tests that verify components work together properly
  - **parameterized_tests.rs**: Tests that run with different parameters

## Current Status

Ferrum is currently in development with the core proxy functionality implemented and tested. The HTTPS inspection and UI components are under active development.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to improve the project.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
