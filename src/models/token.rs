use {
    serde::{Serialize, Deserialize},
    serde_with::serde_as,
    primitive_types::{U256, H160},
    crate::models::u256_decimal::{self, DecimalU256},
};

#[serde_as]
#[derive(Clone, Debug, Default, Serialize)]
pub struct TokenInfoModel {
    pub decimals: Option<u8>,
    pub alias: Option<String>,
    pub external_price: Option<f64>,
    pub normalize_priority: Option<u64>,
    #[serde_as(as = "Option<DecimalU256>")]
    pub internal_buffer: Option<U256>,
    /// Is token in the external list containing only safe tokens
    pub accepted_for_internalization: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TokenAmount {
    #[serde(with = "u256_decimal")]
    pub amount: U256,
    pub token: H160,
}
