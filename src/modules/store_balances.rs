use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::pb::BlockPoolEvents;
use crate::store_key;

#[substreams::handlers::store]
pub fn store_balances(events: BlockPoolEvents, store: StoreAddBigInt) {
    for delta in &events.balance_deltas {
        let key = store_key::balance_str(&delta.pool_address, &delta.token);
        let amount = delta.delta.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
        store.add(0, key, &amount);
    }
}
