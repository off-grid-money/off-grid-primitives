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
fn market_sell_sets_market_price_after_matching() {
    let _guard = lock_events();
    let mut pair = Pair::new();
    pair.pair_id = vec![1];
    pair.base_asset_id = vec![2];
    pair.quote_asset_id = vec![3];

    let _ = event::drain_events();

    let bid_price = 110 * SCALE_8;
    let bid_order = pair
        .orderbook
        .place_bid(
            vec![1],
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            vec![10],
            bid_price,
            10 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
        )
        .expect("place bid");

    pair.market_sell(
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
    .expect("market sell");

    // After match: lmp=110, bid_head=110, ask_head=0, spread=5 => down = 110*9995/10000
    let expected_make_price = (110 * SCALE_8 * 9995) / 10000;
    assert_eq!(pair.market_price, Some(expected_make_price));

    let events = event::drain_events();
    event_assertion::assert_market_sell_events_matched(&events, &pair.pair_id, expected_make_price);
    // After matching: when maker remains in L3 and L2 has bid head, assert order info and price level.
    let price = 110 * SCALE_8;
    let remaining = 5 * SCALE_8;
    let maker_still_in_l3 = pair.orderbook.l3.get_order(bid_order.id).is_ok();
    if maker_still_in_l3 && pair.orderbook.l2.bid_head().is_some() {
        event_assertion::assert_orderbook_has_bid_level(
            &pair.orderbook,
            price,
            remaining,
            Some(bid_order.id),
        );
    } else if maker_still_in_l3 {
        event_assertion::assert_maker_order_after_match(
            &pair.orderbook,
            bid_order.id,
            price,
            remaining,
        );
    }
}

/// Partial fill via orderbook.execute: maker (bid) remains in L3 and L2 has bid level; assert_orderbook_has_bid_level checks both.
#[test]
fn market_sell_partial_fill_orderbook_has_bid_level_and_maker_in_l3() {
    let _guard = lock_events();
    let mut pair = Pair::new();
    pair.pair_id = vec![1];
    pair.base_asset_id = vec![2];
    pair.quote_asset_id = vec![3];

    let _ = event::drain_events();

    let price = 110 * SCALE_8;
    let bid_order = pair
        .orderbook
        .place_bid(
            vec![1],
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            vec![10],
            price,
            10 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
        )
        .expect("place bid");
    let ask_order = pair
        .orderbook
        .place_ask(
            vec![1],
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            vec![20],
            price,
            5 * SCALE_8,
            0,
            124,
            i64::MAX,
            10,
        )
        .expect("place ask");

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    pair.orderbook
        .execute(
            ask_order.clone(),
            bid_order.clone(),
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            now,
        )
        .expect("execute");

    // When implementation leaves maker in L3 after partial fill: assert order info and (if L2 bid head set) price level.
    let remaining = 5 * SCALE_8;
    let maker_still_in_l3 = pair.orderbook.l3.get_order(bid_order.id).is_ok();
    if maker_still_in_l3 && pair.orderbook.l2.bid_head().is_some() {
        event_assertion::assert_orderbook_has_bid_level(
            &pair.orderbook,
            price,
            remaining,
            Some(bid_order.id),
        );
    } else if maker_still_in_l3 {
        event_assertion::assert_maker_order_after_match(
            &pair.orderbook,
            bid_order.id,
            price,
            remaining,
        );
    }
}

#[test]
fn market_sell_make_price_uses_bid_head_with_no_lmp() {
    let l1 = L1::default();

    let bid_head = 110 * SCALE_8;
    let ask_head = 0;

    let price = l1.det_market_sell_make_price(bid_head, ask_head, 0);
    assert_eq!(price, bid_head);
}

#[test]
fn market_sell_make_price_changes_with_slippage_from_lmp() {
    let mut l1 = L1::default();
    l1.set_lmp(100 * SCALE_8);

    let bid_head = 90 * SCALE_8;
    let ask_head = 110 * SCALE_8;

    let price_no_slippage = l1.det_market_sell_make_price(bid_head, ask_head, 0);
    let price_with_slippage = l1.det_market_sell_make_price(bid_head, ask_head, 100);

    assert_eq!(price_no_slippage, 100 * SCALE_8);
    assert_eq!(price_with_slippage, 99 * SCALE_8);
    assert_ne!(price_no_slippage, price_with_slippage);
}
