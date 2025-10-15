use axum::{
    routing::get_service,
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::TraceLayer,
    compression::CompressionLayer,
    cors::CorsLayer,
};
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_env_filter("minicdn=info,tower_http=info")
        .init();

    // Directory to serve static files from
    let serve_dir = ServeDir::new("static").precompressed_gzip();

    // Build app
    let app = Router::new()
        .nest_service("/", get_service(serve_dir))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive());

    // Bind address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8180));
    tracing::info!("🚀 MiniCDN running at http://{}/", addr);

    // Create TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();

    // Start server
    axum::serve(listener, app)
        .await
        .unwrap();
}
