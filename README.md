# Off-Grid Primitives

<p align="center">
  <img src="medias/banner.png" alt="Off-Grid banner" width="100%">
</p>

**off-grid-primitives** is a Rust crate that defines the core data structures and logic for the Off-Grid: spot market state (L1/L2/L3), order books, matching engine, and account balances across spot, futures, options, and money markets.

<p align="center">
  <img src="medias/profile.png" alt="Off-Grid profile" width="120">
</p>

---

## Overview

This library provides the **primitives** used to build Off-Grid trading infrastructure. All types are serializable (e.g. via `serde`) and suitable for use in runtimes, indexers, and off-chain services.

| Component | Path | Description |
|-----------|------|-------------|
| **Spot** | [src/spot/](src/spot/README.md) | Order book, market data (L1/L2/L3), pair state, matching engine, and events |
| **Account** | [src/account/](src/account/README.md) | Balance interfaces for spot, futures, and option accounts |
| **Asset** | [src/asset/](src/asset/README.md) | Asset types and definitions for spot, futures, and option instruments |

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
off-grid-primitives = { path = ".." }  # or from git/crates.io
```

**Spot:**

```rust
use off_grid_primitives::spot::{Pair, MatchingEngine, L1};
```

**Account:**

```rust
use off_grid_primitives::account::{AccountBalances, collect_balances};
```

---

## Testing

From the repository root, run all tests:

```bash
cargo test
```

Run tests by component:

```bash
# Account primitives
cargo test account

# Spot primitives (L1, L2, L3, orderbook, pair, matching engine)
cargo test spot

# Spot sub-components
cargo test spot::l1
cargo test spot::l2
cargo test spot::l3
cargo test spot::orderbook
cargo test spot::pair
cargo test spot::matching_engine

# Orders (cross-component)
cargo test orders
```

Tests live under `tests/` and mirror the crate layout: `tests/account.rs`, `tests/spot/` (with `l1.rs`, `l2.rs`, `l3.rs`, `orderbook/`, `pair/`, `matching_engine/`), and `tests/orders.rs`.

---

## Dependencies

- `serde`, `serde_bytes` — Serialization
- `thiserror` — Error types
- `ulid` — Order ids
- `blake3` — Hashing
- `once_cell` — Lazy/static initialization

---

## Issue template

We use GitHub issue templates for bugs and feature requests:

- **[Bug report](.github/ISSUE_TEMPLATE/bug_report.md)** — For bugs or incorrect behavior.
- **[Feature request](.github/ISSUE_TEMPLATE/feature_request.md)** — For new features or enhancements.

Open an [issue](https://github.com/your-org/off-grid-primitives/issues) and choose the appropriate template.

---

## Contributing

Contributions are welcome. Please read **[CONTRIBUTING.md](CONTRIBUTING.md)** for:

- How to report bugs and suggest features.
- How to submit pull requests.
- Development setup and project structure.

---

## Changelog

Release history and notable changes are documented in **[CHANGELOG.md](CHANGELOG.md)**.

---

## License

See [LICENSE](LICENSE).
