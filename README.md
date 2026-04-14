# Liquidity Party Substreams

Substreams package that indexes [Liquidity Party](https://liquidity.party) LMSR-backed multi-asset pools on Ethereum mainnet and outputs per-pool TVL and APY metrics.

## Outputs

### `map_pool_metrics` (BlockMetrics)

Emitted on every block that has pool activity. Each `PoolMetrics` entry contains:

| Field | Description |
|---|---|
| `balances[]` | Current token balances in raw token units (one per pool token) |
| `fees_24h[]` | LP fees earned in the last ~24h per token |
| `apy_bps[]` | Per-token APY in basis points |
| `quote_tvl` | TVL in token[0] units: `balance[0] × num_tokens` |
| `quote_fees_24h` | 24h fees converted to token[0] using pool-internal pricing |
| `quote_apr_bps` | Simple annualized APR in bps (365.24 days/year) |
| `quote_apy_bps` | Compound APY in bps with daily compounding (365.24 days/year) |

All quote-adjusted values use the pool's own internal price (`balance[0] / balance[n]`), since every pool maintains equally-valued inventory and uses a stablecoin as `token[0]`.

## Usage

```bash
# Build WASM
make build

# Create .spkg bundle
make pack

# Stream against mainnet (requires SUBSTREAMS_API_TOKEN)
make run

# Publish to Substreams.dev registry
make publish
```

## Factory

`PartyPlanner` deployed at `0x7692e502FB8cE1c13A97DbBE380Be05A545ee0a9` (block 24535806).
