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

#[derive(Clone, Debug, Serialize)]
pub enum AmmKind {
    ConstantProduct,
    WeightedProduct,
    Stable
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstantProductReservesModel;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeightedProductReservesModel {
    #[serde(with = "u256_decimal")]
    pub balance: U256,
    #[serde(with = "ratio_as_decimal")]
    pub weight: BigRational,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReservesModel {
    ConstantProduct(ConstantProductReservesModel),
    WeightedProduct(WeightedProductReservesModel),
}

#[derive(Clone, Debug, Serialize)]
pub struct AmmModel {
    kind: AmmKind,
    reserves: BTreeMap<H160, ReservesModel>,
    // #[serde(flatten)]
    // pub parameters: AmmParameters,
    #[serde(with = "ratio_as_decimal")]
    pub fee: BigRational,
    pub cost: TokenAmount,
    // pub mandatory: bool,
    // pub address: H160,
}
