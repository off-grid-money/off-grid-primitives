use off_grid_primitives::spot::event;
use off_grid_primitives::spot::time_in_force::TimeInForce;
use off_grid_primitives::spot::Pair;

use super::event_assertion;
use super::EVENT_MUTEX;

fn lock_events() -> std::sync::MutexGuard<'static, ()> {
    EVENT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

const SCALE_8: u64 = 1_0000_0000;

#[test]
fn limit_buy_moves_lmp_to_best_ask() {
    let _guard = lock_events();
    let mut pair = Pair::new();
    pair.pair_id = vec![1];
    pair.base_asset_id = vec![2];
    pair.quote_asset_id = vec![3];

    let _ = event::drain_events();

    let ask_price = 90 * SCALE_8;
    let _ask_order = pair
        .orderbook
        .place_ask(
            vec![1],
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            vec![10],
            ask_price,
            1 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
        )
        .expect("place ask");

    assert_eq!(pair.orderbook.l2.ask_head(), Some(ask_price));

    pair.limit_buy(
        vec![2],
        None,
        vec![20],
        100 * SCALE_8,
        100 * SCALE_8,
        0,
        124,
        i64::MAX,
        5,
        10,
        TimeInForce::GoodTillCanceled,
    )
    .expect("limit buy");

    assert_eq!(pair.orderbook.l2.ask_head(), None);
    // market_price = make_price after matching: lmp=90, bid_head=100, ask_head=0, spread=5 (maker_fee_bps) => up = 90*10005/10000
    let expected_make_price = (90 * SCALE_8 * 10005) / 10000;
    assert_eq!(pair.market_price, Some(expected_make_price));

    let events = event::drain_events();
    // SpotOrderPlaced for resting bid is from initial place_bid (full amnt/cqty/pqty)
    event_assertion::assert_limit_buy_events_matched(
        &events,
        &pair.pair_id,
        &pair.base_asset_id,
        &pair.quote_asset_id,
        &[20],
        100 * SCALE_8,
        100 * SCALE_8,
        100 * SCALE_8,
        100 * SCALE_8,
        124,
        i64::MAX,
        true,
        Some(expected_make_price),
    );
    // Resting bid is really in the orderbook: L3 has order with price 100, remaining cqty; L2 has level.
    let expected_resting_cqty = 99_99_000000; // 99.99 * SCALE_8 (matching uses base/quote conversion)
    let has_order_placed = events.iter().any(|e| matches!(e,
        event::SpotEvent::SpotOrderPlaced { maker_account_id: m, is_bid: true, .. } if m == &[20]
    ));
    if has_order_placed {
        event_assertion::assert_resting_order_in_orderbook(
            &pair.orderbook,
            &events,
            &[20],
            true,
            100 * SCALE_8,
            expected_resting_cqty,
        );
    } else {
        event_assertion::assert_resting_order_in_orderbook_by_level(
            &pair.orderbook,
            true,
            100 * SCALE_8,
            expected_resting_cqty,
        );
    }
}
