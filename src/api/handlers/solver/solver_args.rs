use {
    primitive_types::H160,
    crate::{
        api::handlers::solver::SolveQueryParams,
        models::auction::{AuctionId, MetadataModel}
    }
};

// TODO: consider refactoring with references over a certain lifetime?
#[derive(Clone, Debug)]
pub(super) struct SolverArgs {
    pub auction_id: Option<AuctionId>,
    pub instance_name: String,
    pub time_limit: u32,
    pub max_nr_exec_orders: u32,
    pub use_internal_buffers: bool,
    pub use_external_prices: bool,
    pub environment: Option<String>,
    pub gas_price: Option<f64>,
    pub native_token: Option<H160>,
}

impl SolverArgs {
    pub fn new(params: &SolveQueryParams, auction_metadata: &MetadataModel) -> Self {
        Self {
            auction_id: params.auction_id.or(auction_metadata.auction_id),
            instance_name: params.instance_name.clone().unwrap_or("Not Provided".to_string()),
            time_limit: params.time_limit.unwrap_or(30),
            max_nr_exec_orders: params.max_nr_exec_orders.unwrap_or(100),
            use_internal_buffers: params.use_internal_buffers.unwrap_or_default(),
            use_external_prices: params.use_external_prices.unwrap_or_default(),
            environment: auction_metadata.environment.clone(),
            gas_price: auction_metadata.gas_price,
            native_token: auction_metadata.native_token,
        }
    }
}
