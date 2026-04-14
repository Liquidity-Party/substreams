// Hand-written prost message structs matching proto/liquidity_party.proto.
// Tag numbers and field types must stay in sync with the .proto file.

use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct PoolMetadata {
    #[prost(string, tag = "1")]
    pub pool_address: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub symbol: String,
    #[prost(string, repeated, tag = "4")]
    pub tokens: Vec<String>,
    #[prost(uint64, tag = "5")]
    pub created_at_block: u64,
    #[prost(uint64, tag = "6")]
    pub created_at_timestamp: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct BalanceDelta {
    #[prost(string, tag = "1")]
    pub pool_address: String,
    #[prost(string, tag = "2")]
    pub token: String,
    /// Signed decimal string. Positive = tokens flowing into pool.
    #[prost(string, tag = "3")]
    pub delta: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct FeeAccrual {
    #[prost(string, tag = "1")]
    pub pool_address: String,
    #[prost(string, tag = "2")]
    pub token: String,
    /// LP fee amount in raw token units (decimal string).
    #[prost(string, tag = "3")]
    pub lp_fee: String,
    /// unix_timestamp / 3600 — used as the snapshot store key suffix.
    #[prost(uint64, tag = "4")]
    pub hour_bucket: u64,
}

#[derive(Clone, PartialEq, Message)]
pub struct BlockPoolEvents {
    #[prost(uint64, tag = "1")]
    pub block_number: u64,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(uint64, tag = "3")]
    pub hour_bucket: u64,
    #[prost(message, repeated, tag = "4")]
    pub new_pools: Vec<PoolMetadata>,
    #[prost(message, repeated, tag = "5")]
    pub balance_deltas: Vec<BalanceDelta>,
    #[prost(message, repeated, tag = "6")]
    pub fee_accruals: Vec<FeeAccrual>,
    /// Pool addresses (hex, no 0x) that emitted Killed this block.
    #[prost(string, repeated, tag = "7")]
    pub killed_pools: Vec<String>,
}

#[derive(Clone, PartialEq, Message)]
pub struct PoolMetrics {
    #[prost(string, tag = "1")]
    pub pool_address: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub symbol: String,
    #[prost(string, repeated, tag = "4")]
    pub tokens: Vec<String>,

    // Raw per-token arrays (one entry per token, same order as tokens[])
    #[prost(string, repeated, tag = "5")]
    pub balances: Vec<String>,
    #[prost(string, repeated, tag = "6")]
    pub fees_24h: Vec<String>,
    #[prost(string, repeated, tag = "7")]
    pub apy_bps: Vec<String>,

    // Quote-adjusted scalars (denominated in tokens[0])
    #[prost(string, tag = "8")]
    pub quote_tvl: String,
    #[prost(string, tag = "9")]
    pub quote_fees_24h: String,
    #[prost(string, tag = "10")]
    pub quote_apr_bps: String,
    #[prost(string, tag = "11")]
    pub quote_apy_bps: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct BlockMetrics {
    #[prost(uint64, tag = "1")]
    pub block_number: u64,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(message, repeated, tag = "3")]
    pub pools: Vec<PoolMetrics>,
}
