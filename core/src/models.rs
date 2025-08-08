use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub key: String,
    pub hwid: String,
}
