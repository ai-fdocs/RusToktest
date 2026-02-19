# rustok-blog module implementation plan (`rustok-blog`)

## Scope and objective

This document captures the current implementation plan for `rustok-blog` in RusToK and
serves as the source of truth for rollout sequencing in `crates/rustok-blog`.

Primary objective: evolve `rustok-blog` in small, testable increments while preserving
compatibility with platform-level contracts.

## Target architecture

- `rustok-blog` remains focused on its bounded context and public crate API.
- Integrations with other modules go through stable interfaces in `rustok-core`
  (or dedicated integration crates where applicable).
- Behavior changes are introduced through additive, backward-compatible steps.
- Observability and operability requirements are part of delivery readiness.

## Delivery phases

### Phase 0 — Foundation ✅ DONE

- [x] Baseline crate/module structure is in place.
- [x] Base docs and registry presence are established.
- [x] Core compile-time integration with the workspace is available.
- [x] Module metadata (slug, name, description, version).
- [x] Empty migrations (wrapper module).

### Phase 1 — Contract hardening ✅ DONE

- [x] Freeze public API expectations for the current module surface.
- [x] Align error/validation conventions with platform guidance.
  - [x] `BlogError` with RichError conversion
  - [x] Helper methods for common errors
- [x] Expand automated tests around core invariants and boundary behavior.
  - [x] Unit tests for state machine
  - [x] Property-based tests for state machine
  - [x] Module metadata tests
  - [x] DTO validation tests
- [x] Define permissions for all resources.
  - [x] Posts (CRUD + Publish)
  - [x] Comments (CRUD + Moderate)
  - [x] Categories (CRUD)
  - [x] Tags (CRUD)
- [x] Type-safe state machine implementation.
  - [x] Draft, Published, Archived states
  - [x] Compile-time safe transitions
  - [x] Comment status with transitions

### Phase 2 — Domain expansion (in progress)

- [ ] Implement prioritized domain capabilities for `rustok-blog`.
  - [x] PostService with CRUD operations
  - [x] State machine integration
  - [ ] CommentService implementation
  - [ ] CategoryService implementation
  - [ ] TagService implementation
- [ ] Standardize cross-module integration points and events.
  - [x] Uses TransactionalEventBus from rustok-outbox
  - [ ] Blog-specific events (PostPublished, CommentCreated, etc.)
- [ ] Document ownership and release gates for new capabilities.
- [ ] Implement full database queries for get_post and list_posts.

### Phase 3 — Productionization (planned)

- [ ] Finalize rollout and migration strategy for incremental adoption.
- [ ] Complete security/tenancy/rbac checks relevant to the module.
- [ ] Validate observability, runbooks, and operational readiness.
- [ ] Performance testing and optimization.
- [ ] Integration tests with test database.

## Current Status

| Component | Status | Notes |
|-----------|--------|-------|
| `lib.rs` | ✅ Complete | Module definition, permissions, exports |
| `error.rs` | ✅ Complete | BlogError with RichError conversion |
| `state_machine.rs` | ✅ Complete | Type-safe post states, comment status |
| `state_machine_proptest.rs` | ✅ Complete | Property-based tests |
| `services/post.rs` | ✅ Core Done | CRUD, publish, archive (queries TODO) |
| `services/comment.rs` | ⬜ TODO | Comment moderation |
| `services/category.rs` | ⬜ TODO | Blog categories |
| `services/tag.rs` | ⬜ TODO | Tag management |
| `dto/post.rs` | ✅ Complete | Create, Update, Response, Query DTOs |
| `entities/` | ✅ Complete | Re-exports from content module |
| Tests (unit) | ✅ Complete | State machine, DTOs, errors |
| Tests (integration) | ⬜ TODO | Requires test database |
| Documentation | ✅ Complete | README, docs, implementation plan |

## Module Contracts

### Public API

```rust
// Main exports
pub use dto::{CreatePostInput, PostResponse, PostSummary, UpdatePostInput};
pub use error::{BlogError, BlogResult};
pub use services::PostService;
pub use state_machine::{
    Archived, BlogPost, BlogPostStatus, CommentStatus, Draft, Published, ToBlogPostStatus,
};
```

### Permissions

```rust
// Posts
Permission::new(Resource::Posts, Action::Create);
Permission::new(Resource::Posts, Action::Read);
Permission::new(Resource::Posts, Action::Update);
Permission::new(Resource::Posts, Action::Delete);
Permission::new(Resource::Posts, Action::List);
Permission::new(Resource::Posts, Action::Publish);

// Comments
Permission::new(Resource::Comments, Action::Create);
Permission::new(Resource::Comments, Action::Read);
Permission::new(Resource::Comments, Action::Update);
Permission::new(Resource::Comments, Action::Delete);
Permission::new(Resource::Comments, Action::List);
Permission::new(Resource::Comments, Action::Moderate);

// Categories & Tags (standard CRUD)
```

### State Transitions

| From | To | Method | Allowed On |
|------|-----|--------|------------|
| Draft | Published | `publish()` | Draft |
| Published | Archived | `archive(reason)` | Published |
| Published | Draft | `unpublish()` | Published |
| Archived | Draft | `restore_to_draft()` | Archived |

## Tracking and updates

When updating `rustok-blog` architecture, API contracts, tenancy behavior, routing,
or observability expectations:

1. Update this file first.
2. Update `crates/rustok-blog/README.md` and `crates/rustok-blog/docs/README.md` when public behavior changes.
3. Update `docs/index.md` links if documentation structure changes.
4. If module responsibilities change, update `docs/modules/registry.md` accordingly.

## Last Updated

2024-02-19 - Phase 1 complete, Phase 2 in progress
