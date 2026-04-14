use substreams::store::{StoreNew, StoreSet, StoreSetInt64};

use crate::pb::BlockPoolEvents;
use crate::store_key;

/// Permanently marks pools that have emitted the `Killed()` event.
/// Uses `set` policy (rather than `set_if_not_exists`) since the kill is idempotent
/// and the pool address is only ever set once anyway.
#[substreams::handlers::store]
pub fn store_killed_pools(events: BlockPoolEvents, store: StoreSetInt64) {
    for pool_addr in &events.killed_pools {
        store.set(0, store_key::killed_str(pool_addr), &1i64);
    }
}
