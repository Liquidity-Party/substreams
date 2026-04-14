use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};

use crate::pb::BlockPoolEvents;
use crate::store_key;

#[substreams::handlers::store]
pub fn store_cumulative_fees(events: BlockPoolEvents, store: StoreAddBigInt) {
    for accrual in &events.fee_accruals {
        let key = store_key::cumulative_fee_str(&accrual.pool_address, &accrual.token);
        let fee = accrual.lp_fee.parse::<BigInt>().unwrap_or_else(|_| BigInt::zero());
        store.add(0, key, &fee);
    }
}
