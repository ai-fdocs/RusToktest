#[test]
fn implementation_plan_tracks_contract_test_coverage() {
    let plan = include_str!("../docs/implementation-plan.md");
    assert!(
        plan.contains("контрактные тесты покрывают все публичные use-case"),
        "implementation plan must include contract test checklist item"
    );
}

#[test]
fn implementation_plan_tracks_checkout_guardrail_visibility() {
    let plan = include_str!("../docs/implementation-plan.md");
    assert!(
        plan.contains("re-entry/release/complete invariants"),
        "implementation plan must keep checkout guardrail visibility markers"
    );
}

