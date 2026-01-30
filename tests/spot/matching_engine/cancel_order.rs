use off_grid_primitives::spot::event::{self, SpotEvent};
use off_grid_primitives::spot::time_in_force::TimeInForce;
use off_grid_primitives::spot::MatchingEngine;

use super::EVENT_MUTEX;

fn lock_events() -> std::sync::MutexGuard<'static, ()> {
    EVENT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

const SCALE_8: u64 = 1_0000_0000;

#[test]
fn cancel_order_emits_spot_order_cancelled() {
    let _guard = lock_events();
    let mut engine = MatchingEngine::new();
    let _ = event::drain_events();

    let pair_id = vec![1];
    engine.add_pair(vec![1], vec![10], vec![11], pair_id.clone(), 1000);
    let _ = event::drain_events();

    // Place two bids at same price so cancelling one leaves the level (avoids PriceMissing after remove_price)
    engine
        .limit_buy(
            vec![1],
            pair_id.clone(),
            None,
            vec![10],
            100 * SCALE_8,
            5 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
            10,
            TimeInForce::GoodTillCanceled,
        )
        .expect("limit_buy");
    let _ = event::drain_events();

    let events = engine
        .limit_buy(
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
        .expect("limit_buy");

    // Use last matching SpotOrderPlaced so we get the order from this limit_buy, not a stale one.
    let order_id_bytes = events
        .iter()
        .filter_map(|e| {
            if let SpotEvent::SpotOrderPlaced { order_id, maker_account_id, .. } = e {
                if maker_account_id == &vec![20] {
                    return Some(order_id.clone());
                }
            }
            None
        })
        .last()
        .expect("SpotOrderPlaced with order_id for owner 20");
    let _ = event::drain_events();

    use off_grid_primitives::spot::orders::OrderId;
    use ulid::Ulid;
    let arr: [u8; 16] = order_id_bytes.as_slice().try_into().expect("order_id 16 bytes");
    let id: OrderId = Ulid::from_bytes(arr);

    let events = engine
        .cancel_order(vec![2], pair_id.clone(), id, vec![20], true)
        .expect("cancel_order");

    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotOrderCancelled { order_id: o, is_bid: true, .. } if o == &order_id_bytes)),
        "expected SpotOrderCancelled: {:?}",
        events
    );
}
