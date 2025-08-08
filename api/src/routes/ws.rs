use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use core::models::AuthRequest;
use core::license::LicenseService;
use std::net::SocketAddr;
use std::sync::Arc;
use storage::repo::DatabaseRepo;

// todo
// pub async fn ws_handler(
//     State(service): State<Arc<LicenseService<DatabaseRepo>>>,
//     ws: WebSocketUpgrade,
//     ConnectInfo(addr): ConnectInfo<SocketAddr>,
//     Path((key, hwid)): Path<(String, String)>,
// ) -> Result<impl IntoResponse, Response> {
//     let req = AuthRequest { key, hwid };
//
//     if let Err(err) = service.auth(req).await {
//         let body = format!("authorization failed: {:?}", err);
//         return Err((StatusCode::UNAUTHORIZED, body).into_response());
//     };
//
//     // Ok(ws.on_upgrade(move |socket| handle_socket(socket, addr)))
// }
