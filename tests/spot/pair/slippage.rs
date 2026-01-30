use off_grid_primitives::spot::market::L1;

const SCALE_8: u64 = 1_0000_0000;

#[test]
fn limit_buy_make_price_changes_with_slippage() {
    let mut l1 = L1::default();
    l1.set_lmp(100 * SCALE_8);

    let bid_head = 99 * SCALE_8;
    let ask_head = 110 * SCALE_8;
    let limit_price = 150 * SCALE_8;

    let price_no_slippage = l1.det_limit_buy_make_price(limit_price, bid_head, ask_head, 0);
    let price_with_slippage = l1.det_limit_buy_make_price(limit_price, bid_head, ask_head, 100);

    assert_eq!(price_no_slippage, 100 * SCALE_8);
    assert_eq!(price_with_slippage, 101 * SCALE_8);
    assert_ne!(price_no_slippage, price_with_slippage);
}

#[test]
fn limit_sell_make_price_changes_with_slippage() {
    let mut l1 = L1::default();
    l1.set_lmp(100 * SCALE_8);

    let bid_head = 90 * SCALE_8;
    let ask_head = 110 * SCALE_8;
    let limit_price = 80 * SCALE_8;

    let price_no_slippage = l1.det_limit_sell_make_price(limit_price, bid_head, ask_head, 0);
    let price_with_slippage = l1.det_limit_sell_make_price(limit_price, bid_head, ask_head, 100);

    assert_eq!(price_no_slippage, 100 * SCALE_8);
    assert_eq!(price_with_slippage, 99 * SCALE_8);
    assert_ne!(price_no_slippage, price_with_slippage);
}
