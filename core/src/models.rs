use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthResponse {
    pub(crate) message: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub(crate) key: String,
    pub(crate) hwid: String,
}
