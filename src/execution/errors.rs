use {
    thiserror::Error
};

#[derive(Error, Debug)]
pub enum OrderError {
    #[error("Invalid execution request for order <{order_id}>: {details}")]
    InvalidExecutionRequest {
        order_id: usize,
        details: String
    }
}


#[derive(Error, Debug)]
pub enum BatchAuctionError {
    #[error("BatchAuctionError: {0}")]
    OrderError(OrderError)
}

impl From<OrderError> for BatchAuctionError {
    fn from(error: OrderError) -> Self {
        Self::OrderError(error)
    }
}
