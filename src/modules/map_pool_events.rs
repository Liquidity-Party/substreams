use std::collections::HashSet;

use substreams::errors::Error;
use substreams::store::{StoreGet, StoreGetProto};
use substreams_ethereum::pb::eth::v2 as eth;

use crate::abi::erc20::events::Transfer;
use crate::abi::pool::events::{BurnSwap, Flash, Killed, Swap, SwapMint};
use crate::pb::{BalanceDelta, BlockPoolEvents, FeeAccrual, PoolMetadata};
use crate::store_key;

#[substreams::handlers::map]
pub fn map_pool_events(
    block: eth::Block,
    pools_store: StoreGetProto<PoolMetadata>,
) -> Result<BlockPoolEvents, Error> {
    let timestamp = block
        .header
        .as_ref()
        .and_then(|h| h.timestamp.as_ref())
        .map(|t| t.seconds as u64)
        .unwrap_or(0);
    let hour_bucket = timestamp / 3600;

    let mut balance_deltas: Vec<BalanceDelta> = Vec::new();
    let mut fee_accruals: Vec<FeeAccrual> = Vec::new();
    let mut killed_pools: Vec<String> = Vec::new();
    // Pools killed in this block — suppress their transfers and fees below.
    let mut killed_this_block: HashSet<String> = HashSet::new();

    // Two-pass per block: first collect Killed events, then process transfers/fees.
    // This ensures that if Killed() appears before or after other events in the same
    // block, we still suppress all activity for that pool within the block.
    for trx in &block.transaction_traces {
        if trx.status != 1 {
            continue;
        }
        for log in trx.receipt.iter().flat_map(|r| r.logs.iter()) {
            if !Killed::match_log(log) {
                continue;
            }
            let pool_key = store_key::pool(&log.address);
            if let Some(pool) = pools_store.get_last(&pool_key) {
                killed_this_block.insert(pool.pool_address.clone());
                killed_pools.push(pool.pool_address.clone());
            }
        }
    }

    for trx in &block.transaction_traces {
        if trx.status != 1 {
            continue;
        }
        let logs: Vec<&eth::Log> = trx.receipt.iter().flat_map(|r| r.logs.iter()).collect();

        for log in &logs {
            // ── ERC-20 Transfer → TVL tracking ──────────────────────────────
            if Transfer::match_log(log) {
                if let Ok(event) = Transfer::decode(log) {
                    handle_transfer(log, &event, &pools_store, &killed_this_block, &mut balance_deltas);
                }
            }

            // ── Pool fee events (skip killed pools) ──────────────────────────
            let pool_key = store_key::pool(&log.address);
            if let Some(pool) = pools_store.get_last(&pool_key) {
                let addr = &pool.pool_address;
                if killed_this_block.contains(addr) {
                    continue;
                }

                if Swap::match_log(log) {
                    if let Ok(event) = Swap::decode(log) {
                        fee_accruals.push(FeeAccrual {
                            pool_address: addr.clone(),
                            token: hex::encode(&event.token_in),
                            lp_fee: event.lp_fee.to_string(),
                            hour_bucket,
                        });
                    }
                } else if SwapMint::match_log(log) {
                    if let Ok(event) = SwapMint::decode(log) {
                        fee_accruals.push(FeeAccrual {
                            pool_address: addr.clone(),
                            token: hex::encode(&event.token_in),
                            lp_fee: event.lp_fee.to_string(),
                            hour_bucket,
                        });
                    }
                } else if BurnSwap::match_log(log) {
                    if let Ok(event) = BurnSwap::decode(log) {
                        fee_accruals.push(FeeAccrual {
                            pool_address: addr.clone(),
                            token: hex::encode(&event.token_out),
                            lp_fee: event.lp_fee.to_string(),
                            hour_bucket,
                        });
                    }
                } else if Flash::match_log(log) {
                    if let Ok(event) = Flash::decode(log) {
                        fee_accruals.push(FeeAccrual {
                            pool_address: addr.clone(),
                            token: hex::encode(&event.token),
                            lp_fee: event.lp_fee.to_string(),
                            hour_bucket,
                        });
                    }
                }
            }
        }
    }

    Ok(BlockPoolEvents {
        block_number: block.number,
        timestamp,
        hour_bucket,
        new_pools: vec![],
        balance_deltas,
        fee_accruals,
        killed_pools,
    })
}

/// Process a Transfer event: if the sender or receiver is a known, non-killed pool,
/// emit a signed balance delta.
fn handle_transfer(
    log: &eth::Log,
    event: &Transfer,
    pools_store: &StoreGetProto<PoolMetadata>,
    killed_this_block: &HashSet<String>,
    balance_deltas: &mut Vec<BalanceDelta>,
) {
    let token_hex = hex::encode(&log.address);

    // Tokens out of pool (pool is the sender)
    if let Some(pool) = pools_store.get_last(&store_key::pool(&event.from)) {
        let addr = &pool.pool_address;
        if !killed_this_block.contains(addr) && pool.tokens.contains(&token_hex) {
            balance_deltas.push(BalanceDelta {
                pool_address: addr.clone(),
                token: token_hex.clone(),
                delta: format!("-{}", event.value),
            });
        }
    }

    // Tokens into pool (pool is the receiver)
    if let Some(pool) = pools_store.get_last(&store_key::pool(&event.to)) {
        let addr = &pool.pool_address;
        if !killed_this_block.contains(addr) && pool.tokens.contains(&token_hex) {
            balance_deltas.push(BalanceDelta {
                pool_address: addr.clone(),
                token: token_hex,
                delta: event.value.to_string(),
            });
        }
    }
}
