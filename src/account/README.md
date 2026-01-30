# Account Primitives

<p align="center">
  <img src="../../medias/banner.png" alt="Off-Grid banner" width="100%">
</p>

Balance interfaces and implementations for spot, futures, and option accounts.

<p align="center">
  <img src="../../medias/profile.png" alt="Off-Grid profile" width="96">
</p>

## Submodules

- **Spot** — `account::spot`
- **Futures** — `account::futures`
- **Option** — `account::option`

## Common Interface

- **`AccountBalances`** — Trait to read `balances()` as `HashMap<Vec<u8>, u64>` (asset id → amount).
- **`collect_balances`** — Aggregates balances from multiple accounts for a set of asset ids.

## Usage

```rust
use off_grid_primitives::account::{AccountBalances, collect_balances};
```
