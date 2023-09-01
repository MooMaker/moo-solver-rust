use serde::{Deserialize};
use {
    axum::{
        http::StatusCode,
        response::IntoResponse,
        Json,
        extract::{State},
    },
    crate::{
        api::Context,
        models::auction::BatchAuctionModel
    },
};
use axum::extract::Path;
use axum::extract::ws::Message;
use futures::SinkExt;

pub async fn solve(State(mut state): State<Context>, Json(payload): Json<BatchAuctionModel>) -> impl IntoResponse
{
    println!("Solve request received");
    println!("Payload: {:?}", payload);
    // Generate RFQ id
    // let rfq_id = uuid::Uuid::new_v4();
    //
    // // Create RFQ
    // let rfq = RFQ {
    //     id: rfq_id.to_string(),
    //     sell_token: payload.sell_token,
    //     buy_token: payload.buy_token,
    //     sell_amount: payload.sell_amount,
    //     buy_amount: payload.buy_amount,
    //     time_limit: payload.time_limit
    // };
    //
    // // Write RFQ to DB
    // let result = state.db.create_rqf(&rfq).await;
    // if let Err(e) = result {
    //     return (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         format!("Failed to place order: {}", e.to_string())
    //     ).into_response();
    // }
    //
    // // Notify makers over websocket
    // let mut makers = state.makers.lock().await;
    // for maker in makers.values_mut() {
    //     println!("Sending RFQ {} to maker", rfq.id);
    //     let _ = maker.send(Message::Text(format!("RFQ {} created", rfq.id))).await;
    // }
    //
    // // Return RFQ
    // (StatusCode::CREATED, Json(rfq)).into_response()
    // (StatusCode::CREATED).into_response()
    StatusCode::CREATED
}
