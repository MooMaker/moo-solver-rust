use {
    std::fmt::Display,
    axum::{
        http::StatusCode,
        response::IntoResponse,
        Json,
        extract::{State, Query}
    },
    serde::{Deserialize},
    crate::{
        api::Context,
        models::auction::{BatchAuctionModel, AuctionId}
    },
    super::solver_args::SolverArgs
};

#[derive(Debug, Deserialize)]
pub struct SolveQueryParams {
    pub instance_name: Option<String>,
    pub time_limit: Option<u32>,
    pub max_nr_exec_orders: Option<u32>,
    pub use_internal_buffers: Option<bool>,
    pub use_external_prices: Option<bool>,
    pub auction_id: Option<AuctionId>
}

pub async fn solve(
    Query(params): Query<SolveQueryParams>,
    State(mut state): State<Context>,
    Json(payload): Json<BatchAuctionModel>,
) -> impl IntoResponse
{
    println!("Received solving request with params: {:?}", &params);

    let solver_args = SolverArgs::new(&params, &payload.metadata);

    println!("Running solver with args: {:?}", &solver_args);
    // println!("Params: {:?}", params);
    // println!("Payload: {:?}", payload);
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

