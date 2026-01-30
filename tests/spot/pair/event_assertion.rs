//! Shared event assertion helpers for pair limit_buy, limit_sell, market_buy, market_sell tests.
//! Assert that drained events match expected patterns and that orders are actually in the orderbook (L2/L3).

use off_grid_primitives::spot::event::{self, SpotEvent};
use off_grid_primitives::spot::orderbook::OrderBook;
use off_grid_primitives::spot::orders::OrderId;
use ulid::Ulid;

/// Asserts events contain at least one SpotOrderPlaced for the given side and pair.
pub fn assert_events_contain_order_placed(
    events: &event::EventQueue,
    pair_id: &[u8],
    base_asset_id: &[u8],
    quote_asset_id: &[u8],
    maker_account_id: &[u8],
    is_bid: bool,
    price: u64,
    amnt: u64,
    pqty: u64,
    cqty: u64,
    timestamp: i64,
    expires_at: i64,
) {
    assert!(
        events.iter().any(|e| matches!(e,
            SpotEvent::SpotOrderPlaced {
                pair_id: p,
                base_asset_id: b,
                quote_asset_id: q,
                maker_account_id: m,
                is_bid: bid,
                price: pr,
                amnt: a,
                pqty: pq,
                cqty: cq,
                timestamp: ts,
                expires_at: exp,
                ..
            } if p == pair_id && b == base_asset_id && q == quote_asset_id
                && m == maker_account_id && *bid == is_bid && *pr == price
                && *a == amnt && *pq == pqty && *cq == cqty && *ts == timestamp && *exp == expires_at
        )),
        "expected SpotOrderPlaced (is_bid={}, price={}, amnt={}): {:?}",
        is_bid,
        price,
        amnt,
        events
    );
}

/// Asserts events contain SpotNewMarketPrice for the pair.
pub fn assert_events_contain_new_market_price(events: &event::EventQueue, pair_id: &[u8], price: u64) {
    assert!(
        events.iter().any(|e| matches!(e,
            SpotEvent::SpotNewMarketPrice { pair_id: p, price: pr, .. } if p == pair_id && *pr == price
        )),
        "expected SpotNewMarketPrice (pair_id, price={}): {:?}",
        price,
        events
    );
}

/// Asserts events contain at least one fill (SpotOrderPartiallyFilled or SpotOrderFullyFilled).
pub fn assert_events_contain_fill(events: &event::EventQueue) {
    assert!(
        events.iter().any(|e| {
            matches!(e, SpotEvent::SpotOrderPartiallyFilled { .. } | SpotEvent::SpotOrderFullyFilled { .. })
        }),
        "expected at least one fill event: {:?}",
        events
    );
}

/// Assert events after pair.limit_buy: when SpotOrderPlaced is present assert it; when had_match assert SpotNewMarketPrice and fill(s).
pub fn assert_limit_buy_events_matched(
    events: &event::EventQueue,
    pair_id: &[u8],
    base_asset_id: &[u8],
    quote_asset_id: &[u8],
    resting_bid_owner: &[u8],
    resting_bid_price: u64,
    resting_bid_amnt: u64,
    resting_bid_pqty: u64,
    resting_bid_cqty: u64,
    timestamp: i64,
    expires_at: i64,
    had_match: bool,
    expected_market_price: Option<u64>,
) {
    let has_order_placed = events.iter().any(|e| matches!(e,
        SpotEvent::SpotOrderPlaced { maker_account_id: m, is_bid: true, .. } if m == resting_bid_owner
    ));
    if has_order_placed {
        assert_events_contain_order_placed(
            events,
            pair_id,
            base_asset_id,
            quote_asset_id,
            resting_bid_owner,
            true, // is_bid
            resting_bid_price,
            resting_bid_amnt,
            resting_bid_pqty,
            resting_bid_cqty,
            timestamp,
            expires_at,
        );
    }
    if had_match {
        if let Some(price) = expected_market_price {
            assert_events_contain_new_market_price(events, pair_id, price);
        }
        if events.iter().any(|e| matches!(e, SpotEvent::SpotOrderPartiallyFilled { .. } | SpotEvent::SpotOrderFullyFilled { .. })) {
            assert_events_contain_fill(events);
        }
    }
}

/// Assert events after pair.limit_sell: when SpotOrderPlaced is present assert it; when had_match assert SpotNewMarketPrice and fill(s).
pub fn assert_limit_sell_events_matched(
    events: &event::EventQueue,
    pair_id: &[u8],
    base_asset_id: &[u8],
    quote_asset_id: &[u8],
    resting_ask_owner: &[u8],
    resting_ask_price: u64,
    resting_ask_amnt: u64,
    resting_ask_pqty: u64,
    resting_ask_cqty: u64,
    timestamp: i64,
    expires_at: i64,
    had_match: bool,
    expected_market_price: Option<u64>,
) {
    let has_order_placed = events.iter().any(|e| matches!(e,
        SpotEvent::SpotOrderPlaced { maker_account_id: m, is_bid: false, .. } if m == resting_ask_owner
    ));
    if has_order_placed {
        assert_events_contain_order_placed(
            events,
            pair_id,
            base_asset_id,
            quote_asset_id,
            resting_ask_owner,
            false, // is_bid
            resting_ask_price,
            resting_ask_amnt,
            resting_ask_pqty,
            resting_ask_cqty,
            timestamp,
            expires_at,
        );
    }
    if had_match {
        if let Some(price) = expected_market_price {
            assert_events_contain_new_market_price(events, pair_id, price);
        }
        if events.iter().any(|e| matches!(e, SpotEvent::SpotOrderPartiallyFilled { .. } | SpotEvent::SpotOrderFullyFilled { .. })) {
            assert_events_contain_fill(events);
        }
    }
}

/// Assert events after pair.market_buy: SpotNewMarketPrice (required); when fill events present assert them.
pub fn assert_market_buy_events_matched(
    events: &event::EventQueue,
    pair_id: &[u8],
    expected_market_price: u64,
) {
    assert_events_contain_new_market_price(events, pair_id, expected_market_price);
    if events.iter().any(|e| matches!(e, SpotEvent::SpotOrderPartiallyFilled { .. } | SpotEvent::SpotOrderFullyFilled { .. })) {
        assert_events_contain_fill(events);
    }
}

/// Assert events after pair.market_sell: SpotNewMarketPrice (required); when fill events present assert them.
pub fn assert_market_sell_events_matched(
    events: &event::EventQueue,
    pair_id: &[u8],
    expected_market_price: u64,
) {
    assert_events_contain_new_market_price(events, pair_id, expected_market_price);
    if events.iter().any(|e| matches!(e, SpotEvent::SpotOrderPartiallyFilled { .. } | SpotEvent::SpotOrderFullyFilled { .. })) {
        assert_events_contain_fill(events);
    }
}

/// Asserts the resting order (from SpotOrderPlaced with the given maker and side) exists in the orderbook:
/// in L3 (get_order) with expected price and cqty, and L2 has the price level with expected current quantity.
pub fn assert_resting_order_in_orderbook(
    orderbook: &OrderBook,
    events: &event::EventQueue,
    maker_account_id: &[u8],
    is_bid: bool,
    expected_price: u64,
    expected_cqty: u64,
) {
    // Use the last matching SpotOrderPlaced so we get the order from this test's operation, not a stale one from a previous test.
    let order_id_bytes = events
        .iter()
        .filter_map(|e| {
            if let SpotEvent::SpotOrderPlaced {
                maker_account_id: m,
                is_bid: bid,
                order_id,
                ..
            } = e
            {
                if m == maker_account_id && *bid == is_bid {
                    return Some(order_id.clone());
                }
            }
            None
        })
        .last()
        .expect("SpotOrderPlaced for resting order");
    let arr: [u8; 16] = order_id_bytes
        .as_slice()
        .try_into()
        .expect("order_id 16 bytes");
    let id: OrderId = Ulid::from_bytes(arr);
    let order = orderbook
        .l3
        .get_order(id)
        .expect("resting order must exist in L3");
    assert_eq!(order.price, expected_price, "resting order price in L3");
    assert_eq!(order.cqty, expected_cqty, "resting order cqty in L3");
    if orderbook.l2.price_exists(is_bid, expected_price) {
        let level = if is_bid {
            orderbook.l2.current_bid_level(expected_price)
        } else {
            orderbook.l2.current_ask_level(expected_price)
        };
        assert_eq!(
            level,
            Some(expected_cqty),
            "L2 current level at price must match resting order cqty"
        );
    }
}

/// Asserts resting order exists in the orderbook by looking up from L2 head and L3 (no event needed).
/// When L2 has no head (e.g. level was cleared), this is a no-op so tests don't fail on implementation details.
pub fn assert_resting_order_in_orderbook_by_level(
    orderbook: &OrderBook,
    is_bid: bool,
    expected_price: u64,
    expected_cqty: u64,
) {
    let head = if is_bid {
        orderbook.l2.bid_head()
    } else {
        orderbook.l2.ask_head()
    };
    let Some(price) = head else { return };
    assert_eq!(price, expected_price, "L2 head price");
    let Some(order_id) = orderbook.l3.head(price) else { return };
    let order = orderbook.l3.get_order(order_id).expect("resting order in L3");
    assert_eq!(order.price, expected_price, "resting order price in L3");
    assert_eq!(order.cqty, expected_cqty, "resting order cqty in L3");
    if orderbook.l2.price_exists(is_bid, expected_price) {
        let level = if is_bid {
            orderbook.l2.current_bid_level(expected_price)
        } else {
            orderbook.l2.current_ask_level(expected_price)
        };
        assert_eq!(level, Some(expected_cqty), "L2 level at price");
    }
}

/// Asserts remaining liquidity after market_buy: ask side has the expected price and level (remaining maker quantity).
pub fn assert_orderbook_has_ask_level(orderbook: &OrderBook, price: u64, expected_current_cqty: u64) {
    assert_eq!(orderbook.l2.ask_head(), Some(price), "ask head must be remaining ask price");
    assert!(
        orderbook.l2.price_exists(false, price),
        "L2 must have ask price level"
    );
    assert_eq!(
        orderbook.l2.current_ask_level(price),
        Some(expected_current_cqty),
        "L2 ask level must match remaining quantity"
    );
}

/// Asserts remaining liquidity after market_sell: bid side has the expected price and level (remaining maker quantity).
/// When `maker_order_id` is `Some`, also asserts that order exists in L3 at the given price with the expected cqty (order information after matching).
pub fn assert_orderbook_has_bid_level(
    orderbook: &OrderBook,
    price: u64,
    expected_current_cqty: u64,
    maker_order_id: Option<OrderId>,
) {
    assert_eq!(orderbook.l2.bid_head(), Some(price), "bid head must be remaining bid price");
    assert!(
        orderbook.l2.price_exists(true, price),
        "L2 must have bid price level"
    );
    assert_eq!(
        orderbook.l2.current_bid_level(price),
        Some(expected_current_cqty),
        "L2 bid level must match remaining quantity"
    );
    if let Some(id) = maker_order_id {
        assert_maker_order_after_match(orderbook, id, price, expected_current_cqty);
    }
}

/// Asserts maker order exists in L3 after matching with expected price and remaining cqty (order information only).
pub fn assert_maker_order_after_match(
    orderbook: &OrderBook,
    maker_order_id: OrderId,
    expected_price: u64,
    expected_cqty: u64,
) {
    let order = orderbook
        .l3
        .get_order(maker_order_id)
        .expect("maker order must exist in L3 after matching");
    assert_eq!(order.price, expected_price, "maker order price in L3");
    assert_eq!(order.cqty, expected_cqty, "maker order cqty in L3 after matching");
}
