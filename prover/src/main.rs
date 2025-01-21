use axum::{middleware, routing::post, serve, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod types;
mod utils;

#[derive(Clone)]
struct AppState {
    api_key: String,
}

#[tokio::main]
async fn main() {
    sp1_sdk::utils::setup_logger();

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
    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app.into_make_service()).await.unwrap();
}
