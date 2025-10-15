# MiniCDN

A lightweight, high-performance Content Delivery Network (CDN) built with Rust and Axum. MiniCDN serves static files with built-in compression, CORS support, and request tracing.

## Features

- 🚀 **High Performance**: Built with Rust and Axum for maximum performance
- 📁 **Static File Serving**: Efficiently serves files from the `static` directory
- 🗜️ **Compression**: Automatic gzip compression with precompressed file support
- 🌐 **CORS Support**: Permissive CORS configuration for cross-origin requests
- 📊 **Request Tracing**: Built-in HTTP request logging and tracing
- ⚡ **Async/Await**: Fully asynchronous request handling with Tokio

## Quick Start

### Prerequisites

- Rust 1.70+ (Edition 2024)
- Cargo

### Installation & Running

1. Clone the repository:
```bash
git clone <repository-url>
cd minicdn
```

2. Build and run the service:
```bash
cargo run
```

The service will start on `http://0.0.0.0:8180`

### Development

For development with auto-reload:
```bash
cargo install cargo-watch
cargo watch -x run
```

## Usage

Once the service is running, you can access your static files through HTTP:

- **Base URL**: `http://localhost:8180/`
- **Static Files**: Place files in the `static/` directory

### Example Files

The service currently includes example files in the `static/` directory:
- `image.jpeg` - Sample image file
- `vid.mp4` - Sample video file  
- `r.md` - Sample markdown file

### Accessing Files

```bash
# Access an image
curl http://localhost:8180/image.jpeg

# Access a video
curl http://localhost:8180/vid.mp4

# Access a markdown file
curl http://localhost:8180/r.md
```

## Configuration

### Port Configuration

The service runs on port `8180` by default. To change the port, modify the `addr` variable in `src/main.rs`:

```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 8180)); // Change 8180 to your desired port
```

### Logging

Logging is configured to show info-level logs for the minicdn service and tower_http middleware. You can adjust the log level by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run  # For debug logs
RUST_LOG=warn cargo run   # For warning logs only
```

### Static Directory

By default, files are served from the `static/` directory. To change this, modify the `serve_dir` configuration in `src/main.rs`:

```rust
let serve_dir = ServeDir::new("your-directory").precompressed_gzip();
```

## Architecture

MiniCDN is built using:

- **[Axum](https://github.com/tokio-rs/axum)**: Modern, ergonomic web framework
- **[Tower HTTP](https://github.com/tower-rs/tower-http)**: HTTP-specific middleware and utilities
- **[Tokio](https://tokio.rs/)**: Asynchronous runtime
- **[Tracing](https://github.com/tokio-rs/tracing)**: Application-level tracing framework

### Middleware Stack

1. **TraceLayer**: HTTP request/response logging
2. **CompressionLayer**: Automatic response compression
3. **CorsLayer**: Cross-Origin Resource Sharing support
4. **ServeDir**: Static file serving with precompressed gzip support

## Performance Features

- **Precompressed Files**: Automatically serves `.gz` files if available
- **Async I/O**: Non-blocking file operations
- **Zero-Copy**: Efficient file streaming
- **HTTP Keep-Alive**: Connection reuse for better performance

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Testing

```bash
cargo test
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Docker Support

Create a `Dockerfile` for containerized deployment:

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/minicdn /usr/local/bin/minicdn
COPY static/ /app/static/
WORKDIR /app
EXPOSE 8180
CMD ["minicdn"]
```

## License

This project is open source. Please add your preferred license.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Roadmap

- [ ] Configuration file support
- [ ] Custom header support
- [ ] Rate limiting
- [ ] File upload endpoints
- [ ] Directory browsing
- [ ] SSL/TLS support
- [ ] Metrics and monitoring endpoints
