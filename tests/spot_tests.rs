// Spot tests share a global event queue (event::drain_events / emit_event).
// All spot tests that use the event queue must use this single mutex so events
// are not drained by another test running in parallel.
// Run with `--test-threads=1` to avoid flaky failures from cross-test event pollution:
//   cargo test -p off-grid-primitives --test spot_tests -- --test-threads=1

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub(crate) static EVENT_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[path = "spot/l1.rs"]
mod l1;
#[path = "spot/l2.rs"]
mod l2;
#[path = "spot/l3.rs"]
mod l3;
#[path = "spot/orderbook/mod.rs"]
mod orderbook;
#[path = "spot/pair/mod.rs"]
mod pair;
#[path = "spot/matching_engine/mod.rs"]
mod matching_engine;