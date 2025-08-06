mod routes;

use crate::routes::licenses::{auth, generate_license};
use axum::Router;
use axum::routing::post;
use core::services::LicenseService;
use std::sync::Arc;
use storage::repo::DatabaseRepo;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    common::logger::init();

    let pool = storage::get_pool().await.unwrap();
    let repo = DatabaseRepo::new(pool);

    let license_service = Arc::new(LicenseService::new(repo));

    let app = Router::new()
        .route("/license/auth", post(auth))
        .route("/license/generate", post(generate_license))
        .layer(TraceLayer::new_for_http())
        .with_state(license_service);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
