use {
    serde::{Deserialize, Serialize},
    primitive_types::{H160, U256},
    crate::models::{
        u256_decimal::{self},
        token::TokenAmount
    }
};

// uid as 56 bytes: 32 for orderDigest, 20 for ownerAddress and 4 for validTo
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct OrderUid(pub [u8; 56]);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderModel {
    pub sell_token: H160,
    pub buy_token: H160,
    #[serde(with = "u256_decimal")]
    pub sell_amount: U256,
    #[serde(with = "u256_decimal")]
    pub buy_amount: U256,
    pub allow_partial_fill: bool,
    pub is_sell_order: bool,
    pub fee: TokenAmount,
    pub cost: TokenAmount,
    pub is_liquidity_order: bool,
    pub has_atomic_execution: Option<bool>,
}
