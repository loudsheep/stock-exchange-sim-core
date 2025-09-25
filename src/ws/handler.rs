use axum::{
    Extension,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};

use crate::{auth::jwt::Claims, AppState};
use redis::AsyncCommands;

pub async fn ws_handler(ws: WebSocketUpgrade, state: Extension<AppState>, _claims: Claims) -> impl IntoResponse {
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
                if let Some(ticker) = text.strip_prefix("subscribe:") {
                    let ticker = ticker.trim().to_uppercase();

                    if !is_valid_ticker(&ticker, &_state).await {
                        let _ = socket
                            .send(Message::Text(
                                format!("Error: Invalid ticker {}", ticker).into(),
                            ))
                            .await;
                        let _ = socket.send(Message::Close(None)).await;
                        break;
                    }

                    // regularly send updates every 1 second
                    let ticker = ticker.clone();
                    let state = _state.clone();
                    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));
                    loop {
                        interval.tick().await;

                        let price = get_price_from_service(&ticker, &state).await;
                        let response = format!("update:{}:{}", ticker, price);

                        if socket.send(Message::Text(response.into())).await.is_err() {
                            tracing::info!("Client disconnected, stopping updates for {}", ticker);
                            break;
                        }
                    }
                } else {
                    let _ = socket
                        .send(Message::Text(
                            "Send subscribe:<TICKER> to start receiving updates".into(),
                        ))
                        .await;
                    continue;
                }
            }
            Message::Close(frame) => {
                tracing::info!("Received close message: {:?}", frame);
                break;
            }
            _ => {}
        }
    }

    tracing::info!("WebSocket connection closed");
}

async fn is_valid_ticker(ticker: &str, _state: &AppState) -> bool {
    // check against redis
    match _state.redis_pool.get().await {
        Ok(mut conn) => match conn.exists::<_, bool>(ticker).await {
            Ok(exists) => exists,
            Err(e) => {
                tracing::error!("Failed to check ticker in redis: {}", e);
                false
            }
        },
        Err(e) => {
            tracing::error!("Failed to get redis connection: {}", e);
            false
        }
    }
}
async fn get_price_from_service(_ticker: &str, _state: &AppState) -> f64 {
    match _state.redis_pool.get().await {
        Ok(mut conn) => match conn.get::<_, f64>(_ticker).await {
            Ok(price) => price,
            Err(e) => {
                tracing::error!("Failed to get price from redis: {}", e);
                0.0
            }
        },
        Err(e) => {
            tracing::error!("Failed to get redis connection: {}", e);
            0.0
        }
    }
}
