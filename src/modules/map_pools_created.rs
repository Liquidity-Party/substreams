use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2 as eth;

use crate::abi::factory::events::PartyStarted;
use crate::pb::{BlockPoolEvents, FeeAccrual, BalanceDelta, PoolMetadata};

#[substreams::handlers::map]
pub fn map_pools_created(
    params: String,
    block: eth::Block,
) -> Result<BlockPoolEvents, Error> {
    let factory_address = parse_factory_address(&params);

    let timestamp = block.header.as_ref().map(|h| h.timestamp.as_ref().map(|t| t.seconds as u64).unwrap_or(0)).unwrap_or(0);
    let hour_bucket = timestamp / 3600;

    let mut new_pools: Vec<PoolMetadata> = Vec::new();

    for trx in &block.transaction_traces {
        if trx.status != 1 { continue; } // only successful txs
        for log in trx.receipt.iter().flat_map(|r| r.logs.iter()) {
            // Filter to factory address only
            if log.address != factory_address { continue; }
            if !PartyStarted::match_log(log) { continue; }

            let event = match PartyStarted::decode(log) {
                Ok(e) => e,
                Err(e) => {
                    substreams::log::info!("Failed to decode PartyStarted: {}", e);
                    continue;
                }
            };

            let pool_hex = hex::encode(&event.pool);
            let token_hexes: Vec<String> = event.tokens.iter().map(|t| hex::encode(t)).collect();

            new_pools.push(PoolMetadata {
                pool_address: pool_hex,
                name: event.name,
                symbol: event.symbol,
                tokens: token_hexes,
                created_at_block: block.number,
                created_at_timestamp: timestamp,
            });
        }
    }

    Ok(BlockPoolEvents {
        block_number: block.number,
        timestamp,
        hour_bucket,
        new_pools,
        balance_deltas: Vec::<BalanceDelta>::new(),
        fee_accruals: Vec::<FeeAccrual>::new(),
        killed_pools: vec![],
    })
}

fn parse_factory_address(params: &str) -> Vec<u8> {
    // params format: "factory_address=7692e502fb8ce1c13a97dbbe380be05a545ee0a9"
    params
        .split('&')
        .find_map(|kv| {
            let mut parts = kv.splitn(2, '=');
            let key = parts.next()?;
            let val = parts.next()?;
            if key.trim() == "factory_address" {
                hex::decode(val.trim()).ok()
            } else {
                None
            }
        })
        .unwrap_or_default()
}
