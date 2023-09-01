use {
    serde::{Serialize},
    primitive_types::{H160, U256},
    crate::models::{
        u256_decimal::{self},
        token::TokenAmount
    }
};

// uid as 56 bytes: 32 for orderDigest, 20 for ownerAddress and 4 for validTo
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct OrderUid(pub [u8; 56]);

#[derive(Clone, Debug, Serialize)]
pub struct OrderModel {
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub id: Option<OrderUid>,
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
    // /// [DEPRECATED] All orders are always mature.
    // pub is_mature: bool,
    // /// [DEPRECATED] Mandatory flag is not useful enough to warrant keeping
    // /// around.
    // #[serde(default)]
    // pub mandatory: bool,
    // /// Signals if the order will be executed as an atomic unit. In that case
    // /// the order's preconditions have to be met for it to be executed
    // /// successfully. This is different from the usual user provided orders
    // /// because those can be batched together and it's only relevant if
    // /// the pre- and post conditions are met after the complete batch got
    // /// executed.
    pub has_atomic_execution: bool,
    // /// [DEPRECATED] CIP-14 risk adjusted solver reward is no longer used
    // pub reward: f64,
}
