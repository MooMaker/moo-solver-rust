use {
    axum::{
        http::StatusCode,
        response::IntoResponse,
        Json,
        extract::{Query}
    },
    serde::{Deserialize},
    crate::{
        models::auction::{BatchAuctionModel, SettledBatchAuctionModel, AuctionId},
        execution::batch_auction::BatchAuction,
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
    Json(payload): Json<BatchAuctionModel>,
) -> impl IntoResponse
{
    println!("Received solving request with params: {:?}", &params);
    println!("Solve batch auction: {:?}", &payload);

    let solver_args = SolverArgs::new(&params, &payload.metadata);

    let mut batch_auction = BatchAuction::from_model(
        solver_args.instance_name.clone(),
        &payload
    );

    println!("Received Batch Auction {}", &batch_auction.get_name());
    println!("Parameters Supplied {:?}", solver_args);

    let result = batch_auction.solve();

    if let Err(e) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to solve batch auction: {}", e.to_string())
        ).into_response();
    }

    let settled_batch_auction = SettledBatchAuctionModel::from(&batch_auction);
    println!("Settled Batch Auction: {:?}", settled_batch_auction);

    (StatusCode::OK, Json(settled_batch_auction))
        .into_response()
}

