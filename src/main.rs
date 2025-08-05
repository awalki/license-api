mod models;
mod schema;

use crate::models::{LicenseKey, NewLicenseKey};
use crate::schema::license_keys::dsl::license_keys;
use crate::schema::license_keys::{hwid, is_activated, key};
use axum::http::HeaderMap;
use axum::{Router, extract::State, http::StatusCode, response::Json, routing::post};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel_async::{
    AsyncPgConnection, RunQueryDsl, pooled_connection::AsyncDieselConnectionManager,
};
use serde::{Deserialize, Serialize};
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let config =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(env::var("DATABASE_URL").unwrap());
    let pool = Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .route("/license/auth", post(auth))
        .route("/license/create", post(create_key))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
pub struct AuthResponse {
    detail: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    key: String,
    hwid: String,
}

async fn auth(
    State(pool): State<Pool>,
    Json(request): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let db_key: LicenseKey = license_keys
        .filter(key.eq(&request.key))
        .select(LicenseKey::as_select())
        .first::<LicenseKey>(&mut conn)
        .await
        .map_err(internal_error)?;

    if let Some(hwid_str) = db_key.hwid {
        if request.hwid != hwid_str {
            return Err((StatusCode::UNAUTHORIZED, "unauthorized".into()));
        };
        if db_key.expires < chrono::Utc::now().naive_utc() {
            return Err((StatusCode::UNAUTHORIZED, "unauthorized".into()));
        }
    } else {
        diesel::update(license_keys.filter(key.eq(&request.key)))
            .set((hwid.eq(&request.hwid), is_activated.eq(true)))
            .execute(&mut conn)
            .await
            .map_err(internal_error)?;
        tracing::debug!("{} successfully activated", db_key.key)
    }

    Ok(Json(AuthResponse {
        detail: "license key is valid".to_string(),
    }))
}

async fn create_key(
    State(pool): State<Pool>,
    headers: HeaderMap,
    Json(new_key): Json<NewLicenseKey>,
) -> Result<Json<LicenseKey>, (StatusCode, String)> {
    if let Some(api_key) = headers.get("x-api-key") {
        if env::var("API_KEY").unwrap() != api_key.to_str().unwrap() {
            return Err((StatusCode::UNAUTHORIZED, "unauthorized".into()));
        }
    }
    let mut conn = pool.get().await.map_err(internal_error)?;

    let res = diesel::insert_into(license_keys::table())
        .values(new_key)
        .returning(LicenseKey::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;

    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
