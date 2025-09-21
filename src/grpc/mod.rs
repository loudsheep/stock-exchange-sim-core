use std::sync::Arc;

use price_feed::PriceRequest;
use tonic::codec;
use tonic::transport::{Channel, channel};
use tonic::{Request, Response, Status};

use price_feed::PriceResponse;
use price_feed::price_feed_server::PriceFeed;

use crate::grpc::price_feed::price_feed_client::PriceFeedClient;
use crate::{AppState, Result};

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
        ticker: "AAPL".into(),
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
        tracing::info!("Received price update: {:?}", update);

        // TODO: save the price update to redis (maybe utilize redis pub/sub here?) or database
    }

    Ok(())
}

// TODO: Implement the gRPC server from this:
// #[derive(Debug, Default)]
// pub struct GrpcClient {}

// #[tonic::async_trait]
// impl PriceFeed for GrpcClient {
//     type StreamPricesStream = codec::Streaming<price_feed::PriceResponse>;

//     async fn get_price(
//         &self,
//         request: Request<PriceRequest>,
//     ) -> Result<Response<PriceResponse>, Status> {
//         // Implement your gRPC client logic here
//         unimplemented!()
//     }

//     async fn stream_prices(
//         &self,
//         request: Request<PriceRequest>,
//     ) -> Result<Response<Self::StreamPricesStream>, Status> {
//         // Implement your gRPC client logic here
//         unimplemented!()
//     }
// }
