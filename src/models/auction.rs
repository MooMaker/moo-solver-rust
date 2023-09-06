use {
    std::collections::BTreeMap,
    serde::{Deserialize, Serialize},
    serde_with::serde_as,
    primitive_types::{H160, U256},
    crate::models::{
        order::{OrderModel, ExecutedOrderModel},
        token::{TokenInfoModel},
        amm::{AmmModel,AmmId},
        u256_decimal::{DecimalU256}
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

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SettledBatchAuctionModel {
    pub orders: BTreeMap<usize, ExecutedOrderModel>,
    // TODO: implement amms
    // #[serde(default)]
    // pub amms: HashMap<usize, UpdatedAmmModel>,
    pub ref_token: H160,
    #[serde_as(as = "BTreeMap<_, DecimalU256>")]
    pub prices: BTreeMap<H160, U256>,
}
