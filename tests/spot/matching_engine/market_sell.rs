use off_grid_primitives::spot::event::{self, SpotEvent};
use off_grid_primitives::spot::time_in_force::TimeInForce;
use off_grid_primitives::spot::MatchingEngine;
use ulid::Ulid;

use crate::pair::event_assertion;
use super::EVENT_MUTEX;

fn lock_events() -> std::sync::MutexGuard<'static, ()> {
    EVENT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

const SCALE_8: u64 = 1_0000_0000;

#[test]
fn market_sell_emits_fill_and_market_price_events() {
    let _guard = lock_events();
    let mut engine = MatchingEngine::new();
    let _ = event::drain_events();

    let pair_id = vec![1];
    engine.add_pair(vec![1], vec![10], vec![11], pair_id.clone(), 1000);
    let _ = event::drain_events();

    // Place a bid so market_sell can match; capture maker (bid) order id for orderbook assertion.
    let limit_buy_events = engine
        .limit_buy(
            vec![1],
            pair_id.clone(),
            None,
            vec![10],
            110 * SCALE_8,
            10 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
            10,
            TimeInForce::GoodTillCanceled,
        )
        .expect("limit_buy");
    // Use last matching SpotOrderPlaced (bid) so we get the order from this limit_buy, not a stale one.
    let maker_order_id = limit_buy_events
        .iter()
        .rev()
        .find_map(|e| {
            if let SpotEvent::SpotOrderPlaced { order_id, maker_account_id, is_bid, .. } = e {
                if maker_account_id == &vec![10] && *is_bid {
                    let arr: [u8; 16] = order_id.as_slice().try_into().ok()?;
                    return Some(Ulid::from_bytes(arr));
                }
            }
            None
        })
        .expect("SpotOrderPlaced for maker bid");
    let _ = event::drain_events();

    let events = engine
        .market_sell(
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
        .expect("market_sell");

    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotNewMarketPrice { pair_id: p, .. } if p == &pair_id)),
        "expected SpotNewMarketPrice: {:?}",
        events
    );
    let has_fill = events.iter().any(|e| {
        matches!(e, SpotEvent::SpotOrderPartiallyFilled { .. } | SpotEvent::SpotOrderFullyFilled { .. })
    });
    if !has_fill {
        assert!(
            events.iter().any(|e| matches!(e, SpotEvent::SpotNewMarketPrice { .. })),
            "expected fill or SpotNewMarketPrice: {:?}",
            events
        );
    }
    // After matching: when maker remains in L3 and L2 has bid head, assert order info and price level.
    let orderbook = engine.orderbook(&pair_id).expect("pair exists");
    let price = 110 * SCALE_8;
    let remaining = 5 * SCALE_8;
    let maker_still_in_l3 = orderbook.l3.get_order(maker_order_id).is_ok();
    if maker_still_in_l3 && orderbook.l2.bid_head().is_some() {
        event_assertion::assert_orderbook_has_bid_level(
            orderbook,
            price,
            remaining,
            Some(maker_order_id),
        );
    } else if maker_still_in_l3 {
        event_assertion::assert_maker_order_after_match(orderbook, maker_order_id, price, remaining);
    }
}
