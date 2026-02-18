# rustok-forum module implementation plan (`rustok-forum`)

## Scope and objective

This document captures the current implementation plan for `rustok-forum` in RusToK and
serves as the source of truth for rollout sequencing in `crates/rustok-forum`.

Primary objective: evolve `rustok-forum` in small, testable increments while preserving
compatibility with platform-level contracts.

## Target architecture

- `rustok-forum` remains focused on its bounded context and public crate API.
- Integrations with other modules go through stable interfaces in `rustok-core`
  (or dedicated integration crates where applicable).
- Behavior changes are introduced through additive, backward-compatible steps.
- Observability and operability requirements are part of delivery readiness.

## Delivery phases

### Phase 0 — Foundation (done)

- [x] Baseline crate/module structure is in place.
- [x] Base docs and registry presence are established.
- [x] Core compile-time integration with the workspace is available.

### Phase 1 — Contract hardening (in progress)

- [x] Freeze public API expectations for the current module surface.
  - Public surface: `CategoryService`, `TopicService`, `ReplyService`, `ModerationService` with CRUD operations.
  - `ModerationService` extended with topic operations: `pin_topic`, `unpin_topic`, `lock_topic`, `unlock_topic`, `close_topic`, `archive_topic`.
- [x] Align error/validation conventions with platform guidance.
  - Empty title/body/content/name/slug in `create` methods return `ForumError::Validation`.
  - Error types follow platform `thiserror` conventions.
- [x] Expand automated tests around core invariants and boundary behavior.
  - 9 inline lib tests for `node_to_topic`, `node_to_category`, `node_to_reply` mapping logic.
  - 15 pure unit tests in `tests/unit.rs`: constants, error display, DTO serde defaults.
  - 2 module contract tests in `tests/module.rs`: metadata and migrations list.
  - Integration test scaffold in `tests/integration.rs` (ignored, requires DB).

### Phase 2 — Domain expansion (planned)

- [ ] Implement prioritized domain capabilities for `rustok-forum`.
- [ ] Standardize cross-module integration points and events.
- [ ] Document ownership and release gates for new capabilities.

### Phase 3 — Productionization (planned)

- [ ] Finalize rollout and migration strategy for incremental adoption.
- [ ] Complete security/tenancy/rbac checks relevant to the module.
- [ ] Validate observability, runbooks, and operational readiness.

## Tracking and updates

When updating `rustok-forum` architecture, API contracts, tenancy behavior, routing,
or observability expectations:

1. Update this file first.
2. Update `crates/rustok-forum/README.md` and `crates/rustok-forum/docs/README.md` when public behavior changes.
3. Update `docs/index.md` links if documentation structure changes.
4. If module responsibilities change, update `docs/modules/registry.md` accordingly.
