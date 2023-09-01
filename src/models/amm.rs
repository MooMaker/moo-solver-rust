use {
    std::collections::BTreeMap,
    serde::{Serialize, Deserialize},
    num::BigRational,
    primitive_types::{H160, U256},
    crate::{
        models::{
            u256_decimal::{self},
            token::TokenAmount,
            ratio_as_decimal
        }
    },
};
pub type AmmId = i64;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AmmKind {
    ConstantProduct,
    WeightedProduct,
    Stable
}

pub type ConstantProductReservesModel = U256;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeightedProductReservesModel {
    #[serde(with = "u256_decimal")]
    pub balance: U256,
    #[serde(with = "ratio_as_decimal")]
    pub weight: BigRational,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReservesModel {
    ConstantProduct(ConstantProductReservesModel),
    WeightedProduct(WeightedProductReservesModel),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AmmModel {
    kind: AmmKind,
    reserves: BTreeMap<H160, ReservesModel>,
    #[serde(with = "ratio_as_decimal")]
    pub fee: BigRational,
    pub cost: TokenAmount,
}
