use std::collections::HashSet;

use substreams::errors::Error;
use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetBigInt, StoreGetInt64, StoreGetProto};

use crate::pb::{BlockMetrics, BlockPoolEvents, PoolMetadata, PoolMetrics};
use crate::store_key;

#[substreams::handlers::map]
pub fn map_pool_metrics(
    events: BlockPoolEvents,
    pools_store: StoreGetProto<PoolMetadata>,
    killed_store: StoreGetInt64,
    balances_store: StoreGetBigInt,
    cumulative_fees_store: StoreGetBigInt,
    fee_snapshots_store: StoreGetBigInt,
) -> Result<BlockMetrics, Error> {
    let current_hour = events.hour_bucket;
    // Snapshot key for 24h ago: fees_24h = cumulative_now - snapshot[hour-24]
    let snapshot_hour = current_hour.saturating_sub(24);

    // Collect pool addresses that had activity this block
    let active_pools: HashSet<String> = events
        .balance_deltas
        .iter()
        .map(|d| d.pool_address.clone())
        .chain(events.fee_accruals.iter().map(|f| f.pool_address.clone()))
        .collect();

    let mut pool_metrics: Vec<PoolMetrics> = Vec::new();

    for pool_addr in &active_pools {
        // Skip killed pools entirely.
        if killed_store.get_last(&store_key::killed_str(pool_addr)).is_some() {
            continue;
        }

        let meta = match pools_store.get_last(&store_key::pool_str(pool_addr)) {
            Some(m) => m,
            None => continue,
        };

        let n = meta.tokens.len();
        if n == 0 {
            continue;
        }

        // ── Per-token raw values ─────────────────────────────────────────────
        let mut balances: Vec<BigInt> = Vec::with_capacity(n);
        let mut fees_24h: Vec<BigInt> = Vec::with_capacity(n);
        let mut apy_bps_strs: Vec<String> = Vec::with_capacity(n);

        for token_hex in &meta.tokens {
            let bal = balances_store
                .get_last(&store_key::balance_str(pool_addr, token_hex))
                .unwrap_or_else(BigInt::zero);

            let cum_now = cumulative_fees_store
                .get_last(&store_key::cumulative_fee_str(pool_addr, token_hex))
                .unwrap_or_else(BigInt::zero);

            let cum_24h_ago = fee_snapshots_store
                .get_last(&store_key::fee_snapshot_str(pool_addr, token_hex, snapshot_hour))
                .unwrap_or_else(BigInt::zero);

            let fee_24h = if cum_now > cum_24h_ago {
                cum_now.clone() - cum_24h_ago
            } else {
                BigInt::zero()
            };

            // Per-token APY in bps: (fee_24h / balance) * 365.24 * 10_000
            let apy_bps = if !bal.is_zero() {
                let daily_rate = bigint_to_f64(&fee_24h) / bigint_to_f64(&bal);
                let apy = (1.0_f64 + daily_rate).powf(365.24) - 1.0;
                format!("{}", (apy * 10_000.0) as i64)
            } else {
                "0".to_string()
            };

            apy_bps_strs.push(apy_bps);
            balances.push(bal);
            fees_24h.push(fee_24h);
        }

        // ── Quote-adjusted metrics (denominated in tokens[0]) ─────────────────
        let balance_0 = &balances[0];

        // quote_tvl = balance[0] * num_tokens
        let quote_tvl = balance_0.clone() * BigInt::from(n as u64);

        // quote_fees_24h = Σ_n( fees_24h[n] * balance[0] / balance[n] )
        let mut quote_fees = BigInt::zero();
        for (fee_n, bal_n) in fees_24h.iter().zip(balances.iter()) {
            if bal_n.is_zero() {
                continue;
            }
            quote_fees = quote_fees + fee_n.clone() * balance_0.clone() / bal_n.clone();
        }

        let (quote_apr_bps, quote_apy_bps) = if !quote_tvl.is_zero() {
            let daily_rate = bigint_to_f64(&quote_fees) / bigint_to_f64(&quote_tvl);
            let apr = daily_rate * 365.24 * 10_000.0;
            let apy = ((1.0_f64 + daily_rate).powf(365.24) - 1.0) * 10_000.0;
            (format!("{}", apr as i64), format!("{}", apy as i64))
        } else {
            ("0".to_string(), "0".to_string())
        };

        pool_metrics.push(PoolMetrics {
            pool_address: pool_addr.clone(),
            name: meta.name.clone(),
            symbol: meta.symbol.clone(),
            tokens: meta.tokens.clone(),
            balances: balances.iter().map(|b| b.to_string()).collect(),
            fees_24h: fees_24h.iter().map(|f| f.to_string()).collect(),
            apy_bps: apy_bps_strs,
            quote_tvl: quote_tvl.to_string(),
            quote_fees_24h: quote_fees.to_string(),
            quote_apr_bps,
            quote_apy_bps,
        });
    }

    Ok(BlockMetrics {
        block_number: events.block_number,
        timestamp: events.timestamp,
        pools: pool_metrics,
    })
}

fn bigint_to_f64(b: &BigInt) -> f64 {
    b.to_string().parse::<f64>().unwrap_or(0.0)
}
