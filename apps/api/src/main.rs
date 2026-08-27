use axum::{
    extract::{Extension, Json},
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct OrderSubmissionPayload {
    pub wallet_address: String,
    pub zk_proof_bytes: String,
    pub order_depth_distribution: Vec<f64>, // Normalized bucket probabilities
}

#[derive(Serialize)]
struct MarketTelemetryResponse {
    pub status: String,
    pub shannon_entropy: f64,
}

// Math Engine: Compute Shannon Entropy of Market Depth
fn calculate_shannon_entropy(probabilities: &[f64]) -> f64 {
    probabilities
        .iter()
        .filter(|&&p| p > 0.0)
        .fold(0.0, |acc, &p| acc - (p * p.log2()))
}

async fn submit_order(
    Json(payload): Json<OrderSubmissionPayload>,
) -> Result<Json<MarketTelemetryResponse>, StatusCode> {
    // 1. Calculate Information-Theoretic Depth Metric
    let entropy = calculate_shannon_entropy(&payload.order_depth_distribution);

    // 2. Return telemetry result (In production: publish ZK proof to Midnight queue)
    Ok(Json(MarketTelemetryResponse {
        status: "PROOF_ACCEPTED".to_string(),
        shannon_entropy: entropy,
    }))
}

#[tokio::main]
async fn main() {
    // Configure CORS for strict production domain security
    let cors = CorsLayer::new()
        .allow_origin("https://app.darkflow.io".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/v1/order", post(submit_order))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
let addr = format!("0.0.0.0:{}", port).parse::<std::net::SocketAddr>().unwrap();
    println!("DarkFlow Gateway listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}