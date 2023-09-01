use {
    serde::{Serialize, Deserialize},
    serde_with::serde_as,
    primitive_types::{H160, U256}
};

/// Pool data in a format prepared for solvers.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct PoolInfo {
    /// Skip serializing address since it's redundant (already serialized
    /// outside of this struct)
    #[serde(skip_serializing)]
    pub address: H160,
    pub tokens: Vec<Token>,
    pub state: PoolState,
    pub gas_stats: PoolStats,
}

/// Pool state in a format prepared for solvers.
#[serde_as]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct PoolState {
    #[serde(with = "u256_decimal")]
    pub sqrt_price: U256,
    #[serde(with = "u256_decimal")]
    pub liquidity: U256,
    #[serde_as(as = "DisplayFromStr")]
    pub tick: BigInt,
    // (tick_idx, liquidity_net)
    #[serde_as(as = "BTreeMap<DisplayFromStr, DisplayFromStr>")]
    pub liquidity_net: BTreeMap<BigInt, BigInt>,
    #[serde(skip_serializing)]
    pub fee: Ratio<u32>,
}

/// Pool stats in a format prepared for solvers
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct PoolStats {
    #[serde(with = "u256_decimal")]
    #[serde(rename = "mean")]
    pub mean_gas: U256,
}
