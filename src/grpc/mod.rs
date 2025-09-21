use std::sync::Arc;

use price_feed::PriceRequest;
use redis::AsyncCommands;
use tonic::transport::Channel;

use crate::{AppState, Result};
use price_feed::price_feed_client::PriceFeedClient;

pub mod price_feed {
    tonic::include_proto!("pricefeed");
}

pub async fn price_updater(state: Arc<AppState>) -> Result<()> {
    let channel = Channel::from_shared(state.config.grpc_server_url.clone())
        .map_err(|e| crate::errors::Error::GrpcError(e.to_string()))?
        .connect()
        .await
        .map_err(|e| crate::errors::Error::GrpcError(e.to_string()))?;

    let mut client = PriceFeedClient::new(channel);

    let request = tonic::Request::new(PriceRequest {
        ticker: "ALL".into(),
    });

    let mut stream = client
        .stream_prices(request)
        .await
        .map_err(|e| crate::errors::Error::GrpcError(e.to_string()))?
        .into_inner();

    while let Some(update) = stream
        .message()
        .await
        .map_err(|e| crate::errors::Error::GrpcError(e.to_string()))?
    {
        // tracing::info!("Received price update: {:?}", update);

        // TODO: save the price update to redis (maybe utilize redis pub/sub here?) or database
        // state
        //     .redis_pool
        //     .get()
        //     .await
        //     .map_err(|e| crate::errors::Error::RedisError(e.to_string()))?
        //     .set::<_, _, ()>(&update.ticker, update.price)
        //     .await
        //     .map_err(|e| crate::errors::Error::RedisError(e.to_string()))?;

        // // publish to a redis channel for subscribers
        let _: () = state
            .redis_pool
            .get()
            .await
            .map_err(|e| crate::errors::Error::RedisError(e.to_string()))?
            .publish(format!("price_update:{}", update.ticker), format!("{}:{}", update.ticker, update.price))
            .await
            .map_err(|e| crate::errors::Error::RedisError(e.to_string()))?;
    }

    Ok(())
}
