use {
    std::collections::BTreeMap,
    serde::{Serialize},
    primitive_types::{H160},
    crate::models::{
        order::OrderModel,
        token::{TokenInfoModel},
        amm::AmmModel
    }
};

pub type AuctionId = i64;

#[derive(Clone, Debug, Serialize, Default)]
pub struct MetadataModel {
    pub environment: Option<String>,
    pub auction_id: Option<AuctionId>,
    // pub run_id: Option<u64>,
    pub gas_price: Option<f64>,
    pub native_token: Option<H160>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct BatchAuctionModel {
    pub tokens: BTreeMap<H160, TokenInfoModel>,
    pub orders: BTreeMap<usize, OrderModel>,
    pub amms: BTreeMap<H160, AmmModel>,
    pub metadata: Option<MetadataModel>,
}
