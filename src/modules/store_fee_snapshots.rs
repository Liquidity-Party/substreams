use substreams::store::{DeltaBigInt, Deltas, StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsBigInt};

use crate::pb::BlockPoolEvents;
use crate::store_key;

/// Captures the cumulative LP fee total at the START of each hourly bucket.
///
/// For each (pool, token) that has a new cumulative fee delta this block,
/// we write `old_value` (the cumulative before this block's fees) keyed by
/// the current hour_bucket — using set_if_not_exists so only the first write
/// per hour succeeds. The 24h fee window is then:
///   fees_24h = cumulative_now - snapshot[current_hour - 24]
///
/// Precision: the snapshot is set on the first fee-bearing block of each hour,
/// so the window error is at most one hour's worth of fees (~4% for a 24h APR).
#[substreams::handlers::store]
pub fn store_fee_snapshots(
    events: BlockPoolEvents,
    cum_fee_deltas: Deltas<DeltaBigInt>,
    store: StoreSetIfNotExistsBigInt,
) {
    let hour = events.hour_bucket;

    for delta in &cum_fee_deltas.deltas {
        // delta.key is "cum:{pool_hex}:{token_hex}"
        // Build snap key: "snap:{pool_hex}:{token_hex}:{hour}"
        if let Some(snap_key) = store_key::fee_snapshot_from_cum_key(&delta.key, hour) {
            // old_value is the cumulative BEFORE this block's fees — correct
            // start-of-hour value (all blocks before the first fee of this hour
            // have no deltas, so old_value equals end-of-previous-hour cumulative).
            store.set_if_not_exists(0, snap_key, &delta.old_value);
        }
    }
}
