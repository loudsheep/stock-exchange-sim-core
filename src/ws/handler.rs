use axum::{
    Extension,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};

use crate::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade, state: Extension<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket, state))
}

async fn handle_connection(mut socket: WebSocket, _state: Extension<AppState>) {
    tracing::info!("New WebSocket connection established");

    if socket
        .send(Message::Text("Welcome to Stock-Sim WebSocket!".into()))
        .await
        .is_err()
    {
        tracing::warn!("Failed to send greeting, client disconnected");
        return;
    }

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                tracing::info!("Received text message: {}", text);
                // Echo the message back
                let _ = socket.send(Message::Text(text)).await;
            }
            Message::Binary(bin) => {
                tracing::info!("Received binary message: {:?}", bin);
                // Echo the message back
                let _ = socket.send(Message::Binary(bin)).await;
            }
            Message::Close(frame) => {
                tracing::info!("Received close message: {:?}", frame);
                break;
            }
            Message::Ping(ping) => {
                tracing::info!("Received ping: {:?}", ping);
                let _ = socket.send(Message::Pong(ping)).await;
            }
            Message::Pong(pong) => {
                tracing::info!("Received pong: {:?}", pong);
            }
        }
    }

    tracing::info!("WebSocket connection closed");
}
