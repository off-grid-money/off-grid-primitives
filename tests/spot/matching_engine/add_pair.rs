use off_grid_primitives::spot::event::{self, SpotEvent};
use off_grid_primitives::spot::MatchingEngine;

use super::EVENT_MUTEX;

fn lock_events() -> std::sync::MutexGuard<'static, ()> {
    EVENT_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn add_pair_emits_spot_pair_added_and_client_account_changed() {
    let _guard = lock_events();
    let mut engine = MatchingEngine::new();
    let _ = event::drain_events();

    let cid = vec![1];
    let admin = vec![10];
    let fee = vec![11];
    let pair_id = vec![2];
    let timestamp = 12345;

    engine.add_pair(cid.clone(), admin.clone(), fee.clone(), pair_id.clone(), timestamp);
    let events = event::drain_events();
    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotPairAdded { cid: c, pair_id: p, timestamp: t } if c == &cid && p == &pair_id && *t == timestamp)),
        "expected SpotPairAdded: {:?}",
        events
    );
    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotPairClientAccountChanged { pair_id: p, cid: Some(c), admin_account_id: Some(a), fee_account_id: Some(f), .. } if p == &pair_id && c == &cid && a == &admin && f == &fee)),
        "expected SpotPairClientAccountChanged: {:?}",
        events
    );
}

#[test]
fn add_pair_client_emits_spot_pair_client_account_changed() {
    let _guard = lock_events();
    let mut engine = MatchingEngine::new();
    let _ = event::drain_events();

    let pair_id = vec![3];
    engine.add_pair(vec![1], vec![10], vec![11], pair_id.clone(), 1000);
    let _ = event::drain_events();

    let events = engine
        .add_pair_client(vec![2], pair_id.clone(), vec![20], vec![21])
        .expect("add_pair_client");

    assert!(
        events.iter().any(|e| matches!(e, SpotEvent::SpotPairClientAccountChanged { pair_id: p, cid: Some(c), admin_account_id: Some(a), fee_account_id: Some(f), .. } if p == &pair_id && c == &vec![2] && a == &vec![20] && f == &vec![21])),
        "expected SpotPairClientAccountChanged: {:?}",
        events
    );
}
