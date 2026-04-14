/// Key builders for all Substreams stores.
/// All addresses are stored as lowercase hex strings without a 0x prefix.

/// Key for store_pools: pool metadata keyed by pool address.
pub fn pool(pool_addr: &[u8]) -> String {
    format!("pool:{}", hex::encode(pool_addr))
}

/// Key for store_pools using an already-encoded hex string.
pub fn pool_str(pool_hex: &str) -> String {
    format!("pool:{}", pool_hex)
}

/// Balance key using already-encoded hex strings.
pub fn balance_str(pool_hex: &str, token_hex: &str) -> String {
    format!("bal:{}:{}", pool_hex, token_hex)
}

/// Cumulative fee key using already-encoded hex strings.
pub fn cumulative_fee_str(pool_hex: &str, token_hex: &str) -> String {
    format!("cum:{}:{}", pool_hex, token_hex)
}

/// Key for store_killed_pools: marks a pool as permanently killed.
pub fn killed_str(pool_hex: &str) -> String {
    format!("killed:{}", pool_hex)
}

/// Snapshot key using already-encoded hex strings (for reading from map_pool_metrics).
pub fn fee_snapshot_str(pool_hex: &str, token_hex: &str, hour_bucket: u64) -> String {
    format!("snap:{}:{}:{}", pool_hex, token_hex, hour_bucket)
}

/// Snapshot key derived from a cumulative fee store key (used in store_fee_snapshots
/// handler which receives keys from store_cumulative_fees deltas).
pub fn fee_snapshot_from_cum_key(cum_key: &str, hour_bucket: u64) -> Option<String> {
    // cum_key format: "cum:{pool_hex}:{token_hex}"
    let rest = cum_key.strip_prefix("cum:")?;
    Some(format!("snap:{}:{}", rest, hour_bucket))
}
