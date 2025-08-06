use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use common::auth::validate_api_key;
use core::models::*;
use core::services::LicenseService;
use std::sync::Arc;
use storage::models::{LicenseKey, NewLicenseKey};
use storage::repo::DatabaseRepo;

pub async fn generate_license(
    State(service): State<Arc<LicenseService<DatabaseRepo>>>,
    headers: HeaderMap,
    Json(req): Json<NewLicenseKey>,
) -> Result<Json<LicenseKey>, (StatusCode, String)> {
    let api_key = headers
        .get("x-api-key")
        .ok_or((StatusCode::UNAUTHORIZED, "unauthorized".to_string()))?;
    validate_api_key(api_key)?;

    Ok(Json(service.generate(req).await?))
}

pub async fn auth(
    State(service): State<Arc<LicenseService<DatabaseRepo>>>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    Ok(Json(service.auth(req).await?))
}
