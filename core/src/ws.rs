use crate::repos::LicenseRepo;
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use std::net::SocketAddr;

pub struct WebSocketService<R: LicenseRepo> {
    repo: R,
}

impl<R: LicenseRepo> WebSocketService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn handle_socket(&self, mut socket: WebSocket, who: SocketAddr) {
        while let Some(Ok(msg)) = socket.next().await {
            match msg {
                Message::Ping(payload) => {
                    println!("[{who}] got ping, replying pong");
                    if let Err(e) = socket.send(Message::Pong(payload)).await {
                        eprintln!("Error sending pong: {e}");
                        break;
                    }
                }
                Message::Pong(_) | Message::Text(_) | Message::Binary(_) | Message::Close(_) => {}
            }
        }
        println!("Websocket context {who} destroyed");
    }
}
