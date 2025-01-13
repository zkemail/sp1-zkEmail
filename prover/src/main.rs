use axum::{routing::post, serve, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new().route("/generate-proof", post(handlers::generate_proof));

    // Get port from environment variable or use default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
