use http::{HeaderValue, StatusCode};
use std::env;

pub fn validate_api_key(api_key: &HeaderValue) -> anyhow::Result<(), (StatusCode, String)> {
    let api_key_from_env = env::var("API_KEY").expect("API_KEY must be set");
    if *api_key != api_key_from_env {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized".to_string()));
    }

    Ok(())
}
