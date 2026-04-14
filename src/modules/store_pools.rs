use substreams::store::{StoreNew, StoreSetIfNotExists, StoreSetIfNotExistsProto};

use crate::pb::{BlockPoolEvents, PoolMetadata};
use crate::store_key;

#[substreams::handlers::store]
pub fn store_pools(events: BlockPoolEvents, store: StoreSetIfNotExistsProto<PoolMetadata>) {
    for pool in &events.new_pools {
        let key = store_key::pool_str(&pool.pool_address);
        store.set_if_not_exists(0, key, pool);
    }
}
