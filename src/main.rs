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
    tracing_subscriber::fmt()
        .with_env_filter("minicdn=info,tower_http=info")
        .init();

    let serve_dir = ServeDir::new("static");

    let app = Router::new()
        .nest_service("/", get_service(serve_dir))
        .layer(TraceLayer::new_for_http()) 
        .layer(CompressionLayer::new())     
        .layer(CorsLayer::permissive());    

    let addr = SocketAddr::from(([0, 0, 0, 0], 8180));
    tracing::info!("🚀 MiniCDN running at http://{}/", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}