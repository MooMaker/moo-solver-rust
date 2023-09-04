use {
    std::collections::BTreeMap,
    serde::{Deserialize, Serialize},
    primitive_types::{H160},
    crate::models::{
        order::OrderModel,
        token::{TokenInfoModel},
        amm::{AmmModel,AmmId}
    }
};

pub type AuctionId = i64;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MetadataModel {
    pub environment: Option<String>,
    pub auction_id: Option<AuctionId>,
    pub gas_price: Option<f64>,
    pub native_token: Option<H160>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BatchAuctionModel {
    pub tokens: BTreeMap<H160, TokenInfoModel>,
    pub orders: BTreeMap<usize, OrderModel>,
    pub amms: BTreeMap<AmmId, AmmModel>,
    pub metadata: MetadataModel,
}
