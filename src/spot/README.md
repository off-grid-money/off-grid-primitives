# Spot Primitives

<p align="center">
  <img src="../../medias/banner.png" alt="Off-Grid banner" width="100%">
</p>

Order book, market data (L1/L2/L3), pair state, and matching engine for spot trading.

<p align="center">
  <img src="../../medias/profile.png" alt="Off-Grid profile" width="96">
</p>

## Market Data Layers

| Layer | Type | Description |
|-------|------|-------------|
| **L1** | `L1` | Top-of-book: last match price, best bid/ask, slippage limits |
| **L2** | `L2`, `Level` | Price levels / depth |
| **L3** | `L3`, `Order`, `Node` | Full order book: orders and linked-list nodes |

## Core Types

- **`Pair`** — A trading pair: `pair_id`, `base_asset_id`, `quote_asset_id`, L1 state, order book, clients, and fee accounts. Supports limit/market buy and sell with events.
- **`MatchingEngine`** — Order matching logic for the spot order book.
- **`OrderBook`** — Order book state and operations.
- **`Order`** — Single order (client id, owner, price, quantity, iceberg, timestamps, fee bps).
- **`TimeInForce`** — Order lifetime (e.g. GTC, IOC, FOK).

## Events

Events are emitted for trades and book changes via `event`.

## Usage

```rust
use off_grid_primitives::spot::{Pair, MatchingEngine, L1, L2, Level, Order, OrderBook};
```
