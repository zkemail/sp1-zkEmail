use axum::{middleware, routing::post, serve, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::FmtSubscriber;

mod handlers;
mod types;
mod utils;

#[derive(Clone)]
struct AppState {
    api_key: String,
}

fn setup_tracing() {
    // Create a simple subscriber with default formatting and info level filter
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO) // Set default level to INFO
        .finish();

    // Initialize the global default subscriber
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

#[tokio::main]
async fn main() {
    // Setup tracing first thing
    setup_tracing();

    tracing::info!("Starting zkEmail prover service");

    let state = AppState {
        api_key: std::env::var("ZKEMAIL_API_KEY").expect("ZKEMAIL_API_KEY must be set"),
    };

    // Middleware to check API key
    async fn auth_middleware(
        state: axum::extract::State<AppState>,
        req: axum::extract::Request,
        next: middleware::Next,
    ) -> Result<axum::response::Response, axum::http::StatusCode> {
        let query = req.uri().query().unwrap_or("");

        if query
            .split('&')
            .any(|param| param == format!("api_key={}", state.api_key))
        {
            Ok(next.run(req).await)
        } else {
            tracing::warn!(uri = %req.uri(), "Unauthorized request attempt");
            Err(axum::http::StatusCode::UNAUTHORIZED)
        }
    }

    // Build our application with a route and middleware
    let app = Router::new()
        .route("/generate-proof", post(handlers::generate_proof))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state);

    // Get port from environment variable or use default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(port = port, "Listening for incoming requests");

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!(error = %err, address = %addr, "Failed to bind to address");
            std::process::exit(1);
        }
    };

    tracing::info!("Server started successfully");

    if let Err(err) = serve(listener, app.into_make_service()).await {
        tracing::error!(error = %err, "Server error occurred");
        std::process::exit(1);
    }
}
