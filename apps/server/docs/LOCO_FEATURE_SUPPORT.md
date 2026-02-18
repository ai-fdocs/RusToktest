# RusToK Server — Loco.rs Feature Matrix (no-duplication baseline)

**Date:** 2026-02-18  
**Loco.rs Version:** `0.16` (workspace)  
**Goal of this doc:** зафиксировать «что берём из Loco», «что оставляем самописом», и где сейчас есть риск дублей.

---

## 1) Decision matrix: Loco vs RusToK custom

> Таблица используется как baseline для ревью: чтобы не дублировать реализацию одного и того же фреймворк-функционала в двух местах.

| Capability area | Loco support | Current implementation in RusToK | Source of truth (target) | Duplication risk | Decision / action |
|---|---|---|---|---|---|
| App lifecycle hooks (`Hooks`) | ✅ | Used directly (`boot`, `routes`, `after_routes`, `truncate`, `register_tasks`, `initializers`, `connect_workers`, `seed`) | **Loco** | Low | Keep on Loco hooks |
| Routing/controllers integration | ✅ | Used through `AppRoutes` + Axum layers | **Loco + project routes** | Low | Keep current |
| Config conventions (`auth`, env yaml, settings extension) | ✅ | Loco config + typed `settings.rustok.*` | **Loco config + typed project extension** | Low | Keep current |
| Tasks (`cargo loco task`) | ✅ | `CleanupTask` registered and used | **Loco Tasks** | Low | Keep current |
| Initializers | ✅ | `TelemetryInitializer` via Loco initializer API | **Loco Initializers** | Low | Keep current |
| Mailer subsystem | ✅ | **Now custom SMTP service (`lettre`)**, used in password reset flow | **Loco Mailer should be source of truth** | **High** | Migrate password reset email flow to Loco Mailer API; keep provider config in `settings.rustok.email.*` if needed |
| Workers / queue jobs | ✅ | **Custom/event-driven**: outbox relay worker started from `connect_workers`; no generic Loco queue workers | **RusToK custom (intentional)** | Medium | Keep custom queue strategy (better extensibility for our event pipeline); do not add parallel Loco queue runtime |
| Storage abstraction (uploads/assets) | ✅ | No unified Loco storage adapter across modules yet | **Loco Storage should be source of truth** | **High** | Introduce shared Loco storage abstraction for all modules to avoid per-module ad-hoc storage |
| Auth business flows (RBAC/session/password reset) | partial (patterns) | Project-specific GraphQL/domain implementation | **RusToK custom domain logic** | Medium | Keep domain logic custom, avoid re-implementing infrastructure layers that Loco already provides |
| Tenant resolution/cache middleware | N/A (project concern) | Custom middleware + cache/backends + invalidation | **RusToK custom** | Low | Keep custom (platform-specific tenancy model) |
| Event bus / outbox transport | N/A (project architecture) | Custom event runtime (memory/outbox/iggy) | **RusToK custom** | Low | Keep custom |

---

## 2) What is implemented from Loco today

### Core hooks and bootstrapping

Implemented in `impl Hooks for App`:
- `app_name`, `app_version`
- `boot` (`create_app`)
- `routes`
- `after_routes`
- `truncate`
- `register_tasks`
- `initializers`
- `connect_workers`
- `seed`

### Tasks and Initializers

- Loco Tasks are active (`tasks::register`, `CleanupTask`).
- Loco Initializers are active (`initializers::create`, `TelemetryInitializer`).

---

## 3) What must be aligned per latest review comments

### 3.1 Mailer must work via Loco

**Current state:** password reset email is sent by custom `EmailService` (`lettre`) from GraphQL mutation.  
**Required alignment:** keep delivery providers/config, but use Loco Mailer API as integration surface.

### 3.2 Workers/queue remains our custom implementation

**Current state:** outbox relay worker is launched from `connect_workers`; no generalized Loco queue worker registry.  
**Decision:** intentional divergence from Loco worker subsystem for extensibility and event-driven architecture.

### 3.3 Loco storage abstraction should be shared across modules

**Current state:** no unified Loco storage layer for uploads/assets module-wide.  
**Required alignment:** add shared storage abstraction via Loco and enforce reuse in all modules that need file/object storage.

---

## 4) Caching: current implementation status (server)

### 4.1 Tenant cache pipeline (current prod path)

Tenant resolution middleware (`middleware/tenant.rs`) implements:
- local cache + negative cache,
- cache key versioning,
- anti-stampede request coalescing (`in_flight` map with `Notify`),
- optional Redis-backed invalidation channel (`tenant.cache.invalidate`),
- metrics counters (hits/misses/negative/coalesced).

### 4.2 Cache backends

`rustok-core` provides shared cache backends:
- `InMemoryCacheBackend` (Moka),
- `RedisCacheBackend` (feature-gated, with circuit breaker).

Server tenant middleware can use Redis cache backend when `redis-cache` feature is enabled; otherwise in-memory backend remains available.

### 4.3 Observability for cache

`/metrics` endpoint exports tenant cache metrics (`rustok_tenant_cache_*`) for hits/misses/entries/negative cache indicators.

### 4.4 Tenant cache V3 status

`tenant_cache_v3.rs` exists as an alternative cache implementation with circuit breaker and Moka cache behavior. It is useful as an R&D/alternative path but current primary integration in app startup/middleware remains through `tenant.rs` infrastructure.

---

## 5) Anti-duplication rules (practical checklist)

Before implementing any new infra feature in server modules:

1. Check if Loco already provides this layer (mailer/storage/tasks/workers).
2. If we intentionally diverge, document **why** (extensibility/performance/architecture) in this file.
3. Do not run two parallel implementations for same integration layer in production path without explicit migration plan.
4. For cache-related changes, ensure metrics and invalidation behavior are defined together with data path.

---

## 6) Sources

- `apps/server/src/app.rs`
- `apps/server/src/tasks/mod.rs`
- `apps/server/src/tasks/cleanup.rs`
- `apps/server/src/initializers/mod.rs`
- `apps/server/src/initializers/telemetry.rs`
- `apps/server/src/services/email.rs`
- `apps/server/src/graphql/auth/mutation.rs`
- `apps/server/src/middleware/tenant.rs`
- `apps/server/src/middleware/tenant_cache_v3.rs`
- `apps/server/src/controllers/metrics.rs`
- `crates/rustok-core/src/cache.rs`
- `apps/server/Cargo.toml`
- `Cargo.toml`
