use off_grid_primitives::spot::event::{self, SpotEvent};
use off_grid_primitives::spot::time_in_force::TimeInForce;
use off_grid_primitives::spot::MatchingEngine;

use crate::pair::event_assertion;
use super::EVENT_MUTEX;

fn lock_events() -> std::sync::MutexGuard<'static, ()> {
    EVENT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

const SCALE_8: u64 = 1_0000_0000;

#[test]
fn market_buy_emits_fill_and_market_price_events() {
    let _guard = lock_events();
    let mut engine = MatchingEngine::new();
    let _ = event::drain_events();

    let pair_id = vec![1];
    engine.add_pair(vec![1], vec![10], vec![11], pair_id.clone(), 1000);
    let _ = event::drain_events();

    // Place an ask so market_buy can match
    engine
        .limit_sell(
            vec![1],
            pair_id.clone(),
            None,
            vec![10],
            90 * SCALE_8,
            10 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
            10,
            TimeInForce::GoodTillCanceled,
        )
        .expect("limit_sell");
    let _ = event::drain_events();

    let events = engine
        .market_buy(
            vec![2],
            pair_id.clone(),
            None,
            vec![20],
            5 * SCALE_8,
            0,
            124,
            i64::MAX,
            5,
            10,
            TimeInForce::GoodTillCanceled,
        )
        .expect("market_buy");

    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotOrderPlaced { .. })),
        "expected at least SpotOrderPlaced: {:?}",
        events
    );
    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotNewMarketPrice { pair_id: p, .. } if p == &pair_id)),
        "expected SpotNewMarketPrice: {:?}",
        events
    );
    let has_fill = events.iter().any(|e| {
        matches!(e, SpotEvent::SpotOrderPartiallyFilled { .. } | SpotEvent::SpotOrderFullyFilled { .. })
    });
    assert!(has_fill, "expected fill event: {:?}", events);
    // Remaining ask (maker) is still in the orderbook: 10 - 5 = 5 at 90.
    let orderbook = engine.orderbook(&pair_id).expect("pair exists");
    event_assertion::assert_orderbook_has_ask_level(orderbook, 90 * SCALE_8, 5 * SCALE_8);
}
