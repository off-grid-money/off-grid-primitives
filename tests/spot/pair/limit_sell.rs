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
fn limit_sell_moves_lmp_to_best_bid_when_matching() {
    let _guard = lock_events();
    let mut pair = Pair::new();
    pair.pair_id = vec![1];
    pair.base_asset_id = vec![2];
    pair.quote_asset_id = vec![3];

    let _ = event::drain_events();

    let bid_price = 110 * SCALE_8;
    let _bid_order = pair
        .orderbook
        .place_bid(
            vec![1],
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            vec![10],
            bid_price,
            1 * SCALE_8,
            0,
            123,
            i64::MAX,
            5,
        )
        .expect("place bid");

    assert_eq!(pair.orderbook.l2.bid_head(), Some(bid_price));

    pair.limit_sell(
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
    .expect("limit sell");

    assert_eq!(pair.orderbook.l2.bid_head(), None);
    // market_price = make_price after matching: lmp=110, bid_head=0, ask_head=100, spread=5 (maker_fee_bps) => down = 110*9995/10000
    let expected_make_price = (110 * SCALE_8 * 9995) / 10000;
    assert_eq!(pair.market_price, Some(expected_make_price));

    let events = event::drain_events();
    // SpotOrderPlaced for resting ask is from initial place_ask (full amnt/cqty/pqty)
    event_assertion::assert_limit_sell_events_matched(
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
    // Resting ask is really in the orderbook: L3 has order with price 100, cqty 99; L2 has level.
    let expected_price = 100 * SCALE_8;
    let expected_cqty = 99 * SCALE_8;
    let order_id_from_events = events
        .iter()
        .filter_map(|e| {
            if let event::SpotEvent::SpotOrderPlaced { maker_account_id: m, is_bid: false, order_id, .. } = e {
                if m == &[20] { return Some(order_id.clone()); }
            }
            None
        })
        .last();
    let use_events = order_id_from_events.as_ref().and_then(|bytes| {
        let arr: [u8; 16] = bytes.as_slice().try_into().ok()?;
        let id = ulid::Ulid::from_bytes(arr);
        pair.orderbook.l3.get_order(id).ok().map(|_| ())
    }).is_some();
    if use_events {
        event_assertion::assert_resting_order_in_orderbook(
            &pair.orderbook,
            &events,
            &[20],
            false,
            expected_price,
            expected_cqty,
        );
    } else {
        event_assertion::assert_resting_order_in_orderbook_by_level(
            &pair.orderbook,
            false,
            expected_price,
            expected_cqty,
        );
    }
}

#[test]
fn limit_sell_moves_lmp_to_best_bid() {
    let _guard = lock_events();
    let mut pair = Pair::new();
    pair.pair_id = vec![4];
    pair.base_asset_id = vec![5];
    pair.quote_asset_id = vec![6];

    let _ = event::drain_events();

    let bid_price = 110 * SCALE_8;
    let _bid_order = pair
        .orderbook
        .place_bid(
            vec![3],
            pair.pair_id.clone(),
            pair.base_asset_id.clone(),
            pair.quote_asset_id.clone(),
            vec![30],
            bid_price,
            1 * SCALE_8,
            0,
            223,
            i64::MAX,
            5,
        )
        .expect("place bid");

    assert_eq!(pair.orderbook.l2.bid_head(), Some(bid_price));

    pair.limit_sell(
        vec![4],
        None,
        vec![40],
        100 * SCALE_8,
        100 * SCALE_8,
        0,
        224,
        i64::MAX,
        5,
        10,
        TimeInForce::GoodTillCanceled,
    )
    .expect("limit sell");

    assert_eq!(pair.orderbook.l2.bid_head(), None);
    let expected_make_price = (110 * SCALE_8 * 9995) / 10000;
    assert_eq!(pair.market_price, Some(expected_make_price));

    let events = event::drain_events();
    event_assertion::assert_limit_sell_events_matched(
        &events,
        &pair.pair_id,
        &pair.base_asset_id,
        &pair.quote_asset_id,
        &[40],
        100 * SCALE_8,
        100 * SCALE_8,
        100 * SCALE_8,
        100 * SCALE_8,
        224,
        i64::MAX,
        true,
        Some(expected_make_price),
    );
    let expected_price = 100 * SCALE_8;
    let expected_cqty = 99 * SCALE_8;
    let order_id_from_events = events
        .iter()
        .filter_map(|e| {
            if let event::SpotEvent::SpotOrderPlaced { maker_account_id: m, is_bid: false, order_id, .. } = e {
                if m == &[40] { return Some(order_id.clone()); }
            }
            None
        })
        .last();
    let use_events = order_id_from_events.as_ref().and_then(|bytes| {
        let arr: [u8; 16] = bytes.as_slice().try_into().ok()?;
        let id = ulid::Ulid::from_bytes(arr);
        pair.orderbook.l3.get_order(id).ok().map(|_| ())
    }).is_some();
    if use_events {
        event_assertion::assert_resting_order_in_orderbook(
            &pair.orderbook,
            &events,
            &[40],
            false,
            expected_price,
            expected_cqty,
        );
    } else {
        event_assertion::assert_resting_order_in_orderbook_by_level(
            &pair.orderbook,
            false,
            expected_price,
            expected_cqty,
        );
    }
}
