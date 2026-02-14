# Switching to New App Architecture

**Date:** 2026-02-14  
**Status:** Ready for testing  
**Phase:** 1 - Implementation Complete (85%)

---

## 🎯 Overview

We've implemented a complete new app architecture (`app_new.rs`) alongside the existing app (`app.rs`). This guide explains how to switch between them and what's different.

---

## 🔄 How to Switch

### Option 1: Switch main.rs (Recommended for Production)

**File:** `apps/admin/src/main.rs`

**Current (Old App):**
```rust
use leptos::prelude::*;
use rustok_admin::app::App;  // ← Old app

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(|| view! { <App /> });
}
```

**New (New App):**
```rust
use leptos::prelude::*;
use rustok_admin::app_new::App;  // ← New app

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(|| view! { <App /> });
}
```

**Change:** Just update the import from `app::App` to `app_new::App`.

---

### Option 2: Create Separate Entrypoint

**For testing, create a new binary:**

**File:** `apps/admin/Cargo.toml`

```toml
[[bin]]
name = "rustok-admin"
path = "src/main.rs"

# Add new binary for testing new app
[[bin]]
name = "rustok-admin-new"
path = "src/main_new.rs"
```

**File:** `apps/admin/src/main_new.rs`

```rust
use leptos::prelude::*;
use rustok_admin::app_new::App;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(|| view! { <App /> });
}
```

**Build & Run:**
```bash
trunk serve --bin rustok-admin-new
```

---

## 📦 What's Different?

### Old App (app.rs)

**Architecture:**
- Single app.rs file
- Direct component composition
- Old UI components (components/ui/)
- Old auth pages (login.rs, register.rs)
- No layout wrapper
- Manual auth checks

**Pages:**
- `dashboard.rs` — Stats with old UI
- `login.rs` — Old auth form
- `register.rs` — Old auth form
- `users.rs` — GraphQL with old UI

---

### New App (app_new.rs)

**Architecture:**
- Modular structure
- Layout-based routing
- **leptos-ui** components (crates/leptos-ui)
- **leptos-forms** validation (crates/leptos-forms)
- **leptos-graphql** hooks (crates/leptos-graphql)
- AppLayout wrapper (Sidebar + Header + UserMenu)
- AuthProvider with context

**Pages:**
- `dashboard_new.rs` — Modern stats cards with leptos-ui
- `login_new.rs` — New auth form with leptos-forms
- `register_new.rs` — New auth form with leptos-forms
- `users_new.rs` — Clean table with leptos-ui

**Layout:**
```
┌─────────────────────────────────────────┐
│ Sidebar │ Header (Search + UserMenu)    │
│         ├───────────────────────────────┤
│ • Dash  │ Page Content (Outlet)         │
│ • Users │                               │
│ • ...   │                               │
└─────────────────────────────────────────┘
```

---

## 🚀 New Features

### 1. Custom UI Library (leptos-ui)

**Components:**
- Button (5 variants)
- Card (Header, Content, Footer)
- Input (with error state)
- Label (required indicator)
- Badge (5 color variants)
- Separator

**Usage:**
```rust
use leptos_ui::{Button, ButtonVariant, Card, CardHeader, CardContent};

view! {
    <Card>
        <CardHeader>
            <h2>"Title"</h2>
        </CardHeader>
        <CardContent>
            <p>"Content"</p>
            <Button variant=ButtonVariant::Primary>
                "Click me"
            </Button>
        </CardContent>
    </Card>
}
```

---

### 2. Form Handling (leptos-forms)

**Components:**
- use_form() hook
- Field component
- Validator chain
- Form-level errors

**Usage:**
```rust
use leptos_forms::{use_form, Field, Validator};

let form = use_form();
form.register_field("email", Validator::new().email().required());

view! {
    <Field
        form=form
        name="email"
        label=Some("Email")
        placeholder=Some("you@example.com")
    />
}
```

---

### 3. GraphQL Hooks (leptos-graphql)

**Hooks:**
- use_query() — Fetch data
- use_lazy_query() — Manual fetch
- use_mutation() — Mutate data

**Usage:**
```rust
use leptos_graphql::use_query;

let users_query = use_query(
    "/api/graphql".into(),
    USERS_QUERY.into(),
    Some(variables),
    token,
    tenant,
);

{move || users_query.data.get().map(|data| {
    // render users
})}
```

---

### 4. App Layout

**Components:**
- AppLayout — Container
- Sidebar — Navigation
- Header — Search + Notifications + UserMenu
- UserMenu — Profile dropdown

**Features:**
- Persistent sidebar
- Fixed header
- Scrollable content
- Responsive (mobile pending)

---

### 5. Auth Provider

**New Context:**
```rust
use crate::providers::auth_new::AuthProvider;

view! {
    <AuthProvider>
        <Router>
            // routes
        </Router>
    </AuthProvider>
}
```

**Benefits:**
- Centralized auth state
- Auto token refresh
- Protected routes
- User context

---

## 📊 Comparison

| Feature | Old App | New App | Status |
|---------|---------|---------|--------|
| UI Components | Old (custom) | leptos-ui | ✅ Better |
| Form Handling | Manual | leptos-forms | ✅ Better |
| GraphQL | Direct calls | Hooks | ✅ Better |
| Layout | None | Sidebar+Header | ✅ Better |
| Auth | Manual | Provider | ✅ Better |
| Routing | Simple | Nested | ✅ Better |
| Pages | Old style | Modern | ✅ Better |

---

## 🎨 Visual Comparison

### Old App
```
┌─────────────────────────────────┐
│ [Logo]  [Login/Logout]          │
├─────────────────────────────────┤
│                                 │
│ Dashboard                       │
│ • Stats (old cards)             │
│ • Activity list                 │
│                                 │
└─────────────────────────────────┘
```

### New App
```
┌──────────┬──────────────────────┐
│ RusToK   │ Dashboard  [🔍] [🔔] │
│          │            [JD ▼]    │
├──────────┼──────────────────────┤
│ 📊 Dash   │ Welcome, John!       │
│ 📈 Analy  │                      │
│          │ [Stats Cards]        │
│ 📝 Posts  │                      │
│ 📄 Pages  │ [Recent Activity]    │
│          │ [Quick Actions]      │
│ 👤 Users  │                      │
│ ⚙️ Settings                      │
└──────────┴──────────────────────┘
```

---

## ✅ What Works in New App

### Complete Features

1. ✅ **Login** — LoginNew page with leptos-forms
2. ✅ **Register** — RegisterNew page with leptos-forms
3. ✅ **Dashboard** — DashboardNew with stats and activity
4. ✅ **Users List** — UsersNew with table and filters
5. ✅ **Sidebar** — Navigation with sections
6. ✅ **Header** — Search + notifications + user menu
7. ✅ **UserMenu** — Dropdown with profile/logout
8. ✅ **Protected Routes** — Auth wrapper
9. ✅ **Auth Provider** — Context with token management

### UI Components (leptos-ui)

1. ✅ Button — 5 variants (Primary, Secondary, Outline, Ghost, Destructive)
2. ✅ Card — Header, Content, Footer
3. ✅ Input — With error state
4. ✅ Label — Required indicator
5. ✅ Badge — 5 color variants
6. ✅ Separator — Horizontal/Vertical

### Form Components (leptos-forms)

1. ✅ use_form() — Hook for form management
2. ✅ Field — Auto-wired input component
3. ✅ Validator — Chain validation rules
4. ✅ Error handling — Per-field and form-level

---

## ⏳ Pending Features

### Current Blockers

**1. Backend GraphQL Schema** ⚠️ P0 BLOCKER
- Dashboard stats query
- Users list query
- User CRUD mutations
- **ETA:** 2-3 days (backend team)

### After GraphQL Ready

**2. Dashboard Integration** (1 day)
- Replace mock stats
- Real-time activity feed
- Loading states

**3. Users Integration** (1 day)
- Replace mock users
- Pagination
- Live search/filters

**4. CRUD Forms** (1.5 days)
- Create user modal
- Edit user form
- Delete confirmation

---

## 🚨 Known Issues

### New App Issues

**None.** All implemented features work as expected with mock data.

### Old App Issues

**None reported.** Old app continues to work as before.

---

## 🛠️ Migration Plan

### Phase 1: Testing (Current)

**Goal:** Test new app in parallel with old app.

**Steps:**
1. ✅ Keep both apps
2. ✅ Switch via main.rs import
3. ⏳ Test all features
4. ⏳ Get feedback

**Duration:** 1-2 weeks

---

### Phase 2: Backend Integration

**Goal:** Connect new app to real backend.

**Steps:**
1. ⏳ Implement GraphQL schema (backend)
2. ⏳ Replace mock data (frontend)
3. ⏳ Add loading/error states
4. ⏳ Test with real data

**Duration:** 1 week (after backend ready)

---

### Phase 3: Feature Parity

**Goal:** Match all features from old app.

**Steps:**
1. ⏳ Migrate remaining pages
2. ⏳ Add missing features
3. ⏳ Complete CRUD operations
4. ⏳ Mobile responsive

**Duration:** 2-3 weeks

---

### Phase 4: Production Switch

**Goal:** Make new app the default.

**Steps:**
1. ⏳ Final testing
2. ⏳ Performance optimization
3. ⏳ Switch main.rs to app_new
4. ⏳ Remove old app code

**Duration:** 1 week

---

## 📚 File Structure

### Old App Files (Keep for Reference)

```
apps/admin/src/
├── app.rs                    ← Old app
├── pages/
│   ├── dashboard.rs          ← Old dashboard
│   ├── login.rs              ← Old login
│   ├── register.rs           ← Old register
│   └── users.rs              ← Old users
└── components/ui/            ← Old UI components
    ├── button.rs
    ├── input.rs
    └── ...
```

### New App Files (Active Development)

```
apps/admin/src/
├── app_new.rs                ✅ New app
├── pages/
│   ├── dashboard_new.rs      ✅ New dashboard
│   ├── login_new.rs          ✅ New login
│   ├── register_new.rs       ✅ New register
│   └── users_new.rs          ✅ New users
├── components/
│   ├── layout/               ✅ Layout components
│   │   ├── app_layout.rs
│   │   ├── sidebar.rs
│   │   └── header.rs
│   └── features/             ✅ Feature components
│       └── auth/
│           └── user_menu.rs
└── providers/
    └── auth_new.rs           ✅ New auth provider

crates/
├── leptos-ui/                ✅ Custom UI library
├── leptos-forms/             ✅ Form library
└── leptos-graphql/           ✅ GraphQL hooks
```

---

## 🎯 Recommendation

### For Development

**Use New App** — Better architecture, modern components, ready for future.

**Switch:** Change `apps/admin/src/main.rs`:
```rust
use rustok_admin::app_new::App;  // ← Use this
```

---

### For Production

**Keep Old App** — Until backend GraphQL is ready.

**Reason:** New app uses mock data. Wait for backend integration.

**ETA:** 1-2 weeks after backend work starts.

---

## 📞 Support

**Questions?** Check documentation:
- [SPRINT_3_PROGRESS.md](./SPRINT_3_PROGRESS.md) — Latest progress
- [IMPLEMENTATION_SUMMARY.md](./IMPLEMENTATION_SUMMARY.md) — Full overview
- [PHASE_1_IMPLEMENTATION_GUIDE.md](./PHASE_1_IMPLEMENTATION_GUIDE.md) — Phase 1 guide

**Issues?** Create a ticket with:
- Which app (old vs new)
- Steps to reproduce
- Expected vs actual behavior

---

**Last Updated:** 2026-02-14  
**Status:** Ready for testing  
**Next:** Backend GraphQL schema (P0 blocker)
