use axum::{extract::{State, ws::{WebSocketUpgrade, WebSocket, Message}}, response::IntoResponse};
use std::sync::Arc;
use crate::api::ServerState;
use futures::{SinkExt, StreamExt};

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<ServerState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut event_rx = state.event_bus.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(event) = event_rx.recv().await {
            if let Ok(json) = serde_json::to_string(&event) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    tracing::debug!("Received WS message: {}", text);
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => { recv_task.abort(); },
        _ = (&mut recv_task) => { send_task.abort(); },
    }
}
