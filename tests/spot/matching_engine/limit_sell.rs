use off_grid_primitives::spot::event::{self, SpotEvent};
use off_grid_primitives::spot::time_in_force::TimeInForce;
use off_grid_primitives::spot::MatchingEngine;

use crate::pair::event_assertion;
use super::EVENT_MUTEX;

fn lock_events() -> std::sync::MutexGuard<'static, ()> {
    EVENT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

const SCALE_8: u64 = 1_0000_0000;

/// limit_sell on MatchingEngine returns events; assert SpotOrderPlaced (ask) is emitted.
#[test]
fn limit_sell_emits_spot_order_placed() {
    let _guard = lock_events();
    let mut engine = MatchingEngine::new();
    let _ = event::drain_events();

    let pair_id = vec![1];
    engine.add_pair(vec![1], vec![10], vec![11], pair_id.clone(), 1000);
    let _ = event::drain_events();

    let events = engine
        .limit_sell(
            vec![2],
            pair_id.clone(),
            None,
            vec![20],
            100 * SCALE_8,
            10 * SCALE_8,
            0,
            124,
            i64::MAX,
            5,
            10,
            TimeInForce::GoodTillCanceled,
        )
        .expect("limit_sell");

    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotOrderPlaced { pair_id: p, is_bid: false, price: pr, amnt: a, .. } if p == &pair_id && *pr == 100 * SCALE_8 && *a == 10 * SCALE_8)),
        "expected SpotOrderPlaced (ask): {:?}",
        events
    );
    // Resting ask is really in the orderbook (L2/L3).
    let orderbook = engine.orderbook(&pair_id).expect("pair exists");
    event_assertion::assert_resting_order_in_orderbook(
        orderbook,
        &events,
        &[20],
        false,
        100 * SCALE_8,
        10 * SCALE_8,
    );
}
