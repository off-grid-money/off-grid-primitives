use off_grid_primitives::spot::event;
use off_grid_primitives::spot::market::L1;
use off_grid_primitives::spot::time_in_force::TimeInForce;
use off_grid_primitives::spot::Pair;

use super::event_assertion;
use super::EVENT_MUTEX;

fn lock_events() -> std::sync::MutexGuard<'static, ()> {
    EVENT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

const SCALE_8: u64 = 1_0000_0000;

#[test]
fn market_buy_sets_market_price_after_matching() {
    let _guard = lock_events();
    let mut pair = Pair::new();
    pair.pair_id = vec![1];
    pair.base_asset_id = vec![2];
    pair.quote_asset_id = vec![3];

    let _ = event::drain_events();

    let ask_price = 110 * SCALE_8;
    let _ask_order = pair
        .orderbook
        .place_ask(
            vec![1],
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            vec![10],
            ask_price,
            10 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
        )
        .expect("place ask");

    pair.market_buy(
        vec![2],
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
    .expect("market buy");

    // After match: buy fully filled, lmp=110, bid_head=0, ask_head=110 (remaining ask). make_price = min(ask_head, up) = 110*SCALE_8
    assert_eq!(pair.market_price, Some(110 * SCALE_8));

    let events = event::drain_events();
    event_assertion::assert_market_buy_events_matched(&events, &pair.pair_id, 110 * SCALE_8);
    // Remaining ask (maker) is still in the orderbook: 10 - 5 = 5 at 110.
    event_assertion::assert_orderbook_has_ask_level(&pair.orderbook, 110 * SCALE_8, 5 * SCALE_8);
}

#[test]
fn market_buy_make_price_uses_ask_head_with_no_lmp() {
    let l1 = L1::default();

    let bid_head = 0;
    let ask_head = 110 * SCALE_8;

    let price = l1.det_market_buy_make_price(bid_head, ask_head, 0);
    assert_eq!(price, ask_head);
}

#[test]
fn market_buy_make_price_changes_with_slippage_from_lmp() {
    let mut l1 = L1::default();
    l1.set_lmp(100 * SCALE_8);

    let bid_head = 90 * SCALE_8;
    let ask_head = 110 * SCALE_8;

    let price_no_slippage = l1.det_market_buy_make_price(bid_head, ask_head, 0);
    let price_with_slippage = l1.det_market_buy_make_price(bid_head, ask_head, 100);

    assert_eq!(price_no_slippage, 100 * SCALE_8);
    assert_eq!(price_with_slippage, 101 * SCALE_8);
    assert_ne!(price_no_slippage, price_with_slippage);
}
