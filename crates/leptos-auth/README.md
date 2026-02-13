# leptos-auth

Authentication library for Leptos applications with JWT support, localStorage persistence, and React Context-like state management.

## Features

- 🔐 **JWT Authentication** — Full auth flow with token management
- 💾 **Persistent Storage** — Auto-save to localStorage
- 🎣 **React-like Hooks** — `use_auth()`, `use_current_user()`, `use_session()`
- 🛡️ **Protected Routes** — `<ProtectedRoute>` and `<GuestRoute>` components
- 🔄 **Auto Refresh** — Token refresh support
- 🌐 **Multi-tenant** — Built-in tenant support
- ⚡ **Reactive** — Leptos signals for real-time updates

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-auth = { path = "../../crates/leptos-auth" }
```

## Quick Start

### 1. Wrap your app with `AuthProvider`

```rust
use leptos::*;
use leptos_auth::AuthProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AuthProvider>
            <Router>
                <Routes>
                    // Your routes here
                </Routes>
            </Router>
        </AuthProvider>
    }
}
```

### 2. Use auth hooks in components

```rust
use leptos::*;
use leptos_auth::{use_auth, use_current_user};

#[component]
pub fn Profile() -> impl IntoView {
    let auth = use_auth();
    let user = use_current_user();

    view! {
        <div>
            <h1>"Welcome, " {move || user.get().map(|u| u.email).unwrap_or_default()}</h1>
            <button on:click=move |_| {
                spawn_local(async move {
                    let _ = auth.sign_out().await;
                });
            }>
                "Sign Out"
            </button>
        </div>
    }
}
```

### 3. Protect routes

```rust
use leptos::*;
use leptos_router::*;
use leptos_auth::ProtectedRoute;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/login" view=LoginPage />
            
            // Protected route - redirects to /login if not authenticated
            <Route path="/dashboard" view=move || view! {
                <ProtectedRoute>
                    <DashboardPage />
                </ProtectedRoute>
            } />
        </Routes>
    }
}
```

## API Reference

### Context & Provider

#### `AuthProvider`

Provides auth context to the component tree.

```rust
#[component]
pub fn AuthProvider(children: Children) -> impl IntoView
```

**Example:**
```rust
view! {
    <AuthProvider>
        <App />
    </AuthProvider>
}
```

---

### Hooks

#### `use_auth()`

Returns the `AuthContext` with full control over authentication.

```rust
pub fn use_auth() -> AuthContext
```

**Methods:**
- `sign_in(email, password, tenant) -> Result<(), AuthError>` — Sign in user
- `sign_up(email, password, name, tenant) -> Result<(), AuthError>` — Register new user
- `sign_out() -> Result<(), AuthError>` — Sign out and clear session
- `refresh_session() -> Result<(), AuthError>` — Refresh JWT token
- `fetch_current_user() -> Result<(), AuthError>` — Fetch user info from API
- `is_authenticated() -> bool` — Check if user is logged in
- `get_token() -> Option<String>` — Get current JWT token
- `get_tenant() -> Option<String>` — Get current tenant

**Signals:**
- `user: RwSignal<Option<AuthUser>>` — Current user data
- `session: RwSignal<Option<AuthSession>>` — Current session (token + tenant)
- `is_loading: RwSignal<bool>` — Loading state
- `error: RwSignal<Option<String>>` — Error message

**Example:**
```rust
let auth = use_auth();

let sign_in_action = create_action(move |input: &(String, String)| {
    let (email, password) = input.clone();
    let auth = auth.clone();
    async move {
        auth.sign_in(email, password, "default".to_string()).await
    }
});
```

---

#### `use_current_user()`

Returns a reactive signal with current user data.

```rust
pub fn use_current_user() -> Signal<Option<AuthUser>>
```

**Example:**
```rust
let user = use_current_user();

view! {
    <Show when=move || user.get().is_some()>
        <p>"Email: " {move || user.get().unwrap().email}</p>
    </Show>
}
```

---

#### `use_session()`

Returns a reactive signal with current session (token + tenant).

```rust
pub fn use_session() -> Signal<Option<AuthSession>>
```

---

#### `use_is_authenticated()`

Returns a reactive boolean signal indicating auth status.

```rust
pub fn use_is_authenticated() -> Signal<bool>
```

**Example:**
```rust
let is_authenticated = use_is_authenticated();

view! {
    <Show when=move || is_authenticated.get()>
        <DashboardLink />
    </Show>
}
```

---

#### `use_is_loading()`

Returns loading state signal.

```rust
pub fn use_is_loading() -> Signal<bool>
```

---

#### `use_auth_error()`

Returns error message signal.

```rust
pub fn use_auth_error() -> Signal<Option<String>>
```

---

#### `use_token()`

Returns current JWT token.

```rust
pub fn use_token() -> Signal<Option<String>>
```

---

#### `use_tenant()`

Returns current tenant slug.

```rust
pub fn use_tenant() -> Signal<Option<String>>
```

---

### Components

#### `ProtectedRoute`

Redirects unauthenticated users to login page.

```rust
#[component]
pub fn ProtectedRoute(
    children: Children,
    #[prop(optional)] redirect_path: Option<String>,
) -> impl IntoView
```

**Props:**
- `children` — Content to show when authenticated
- `redirect_path` — Where to redirect (default: `/login`)

**Example:**
```rust
<ProtectedRoute redirect_path="/auth/signin".to_string()>
    <AdminPanel />
</ProtectedRoute>
```

---

#### `GuestRoute`

Redirects authenticated users to dashboard (opposite of `ProtectedRoute`).

```rust
#[component]
pub fn GuestRoute(
    children: Children,
    #[prop(optional)] redirect_path: Option<String>,
) -> impl IntoView
```

**Example:**
```rust
<GuestRoute redirect_path="/dashboard".to_string()>
    <LoginPage />
</GuestRoute>
```

---

#### `RequireAuth`

Shows children only if authenticated, otherwise shows fallback.

```rust
#[component]
pub fn RequireAuth(
    children: Children,
    #[prop(optional)] fallback: Option<View>,
) -> impl IntoView
```

**Example:**
```rust
<RequireAuth fallback=view! { <p>"Please sign in"</p> }.into_view()>
    <SecretContent />
</RequireAuth>
```

---

### Types

#### `AuthUser`

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
}
```

---

#### `AuthSession`

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthSession {
    pub token: String,
    pub tenant: String,
}
```

---

#### `AuthError`

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
pub enum AuthError {
    Unauthorized,
    InvalidCredentials,
    Network,
    Http(u16),
}
```

---

### Storage Helpers

Low-level functions for manual storage management (usually not needed).

```rust
pub fn save_session(session: &AuthSession) -> Result<(), AuthError>
pub fn load_session() -> Result<AuthSession, AuthError>
pub fn save_user(user: &AuthUser) -> Result<(), AuthError>
pub fn load_user() -> Result<AuthUser, AuthError>
pub fn clear_session()
pub fn get_token() -> Option<String>
pub fn get_tenant() -> Option<String>
```

---

### API Functions

Low-level HTTP functions (usually not needed, use `AuthContext` instead).

```rust
pub async fn sign_in(email: String, password: String, tenant: String) 
    -> Result<(AuthUser, AuthSession), AuthError>

pub async fn sign_up(email: String, password: String, name: Option<String>, tenant: String)
    -> Result<(AuthUser, AuthSession), AuthError>

pub async fn sign_out(token: &str) -> Result<(), AuthError>

pub async fn get_current_user(token: &str) -> Result<AuthUser, AuthError>

pub async fn forgot_password(email: String) -> Result<(), AuthError>

pub async fn reset_password(token: String, new_password: String) -> Result<(), AuthError>

pub async fn refresh_token(token: &str) -> Result<String, AuthError>
```

---

## Complete Example: Login Page

```rust
use leptos::*;
use leptos_router::*;
use leptos_auth::{use_auth, GuestRoute};

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    
    let submit = create_action(move |_: &()| {
        let auth = auth.clone();
        let email = email.get();
        let password = password.get();
        
        async move {
            set_error.set(None);
            
            match auth.sign_in(email, password, "default".to_string()).await {
                Ok(_) => {
                    navigate("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error.set(Some(format!("{:?}", e)));
                }
            }
        }
    });
    
    view! {
        <GuestRoute>
            <div class="min-h-screen flex items-center justify-center">
                <form on:submit=move |e| {
                    e.prevent_default();
                    submit.dispatch(());
                } class="w-full max-w-md space-y-4">
                    <h1 class="text-2xl font-bold">"Sign In"</h1>
                    
                    <Show when=move || error.get().is_some()>
                        <div class="bg-red-100 text-red-700 p-3 rounded">
                            {move || error.get().unwrap_or_default()}
                        </div>
                    </Show>
                    
                    <input
                        type="email"
                        placeholder="Email"
                        class="w-full px-4 py-2 border rounded"
                        on:input=move |e| set_email.set(event_target_value(&e))
                        prop:value=email
                    />
                    
                    <input
                        type="password"
                        placeholder="Password"
                        class="w-full px-4 py-2 border rounded"
                        on:input=move |e| set_password.set(event_target_value(&e))
                        prop:value=password
                    />
                    
                    <button
                        type="submit"
                        class="w-full bg-blue-600 text-white py-2 rounded hover:bg-blue-700"
                        disabled=move || submit.pending().get()
                    >
                        {move || if submit.pending().get() { "Signing in..." } else { "Sign In" }}
                    </button>
                </form>
            </div>
        </GuestRoute>
    }
}
```

---

## Backend API Requirements

The library expects the following REST endpoints:

| Method | Endpoint | Request Body | Response |
|--------|----------|--------------|----------|
| POST | `/api/auth/login` | `{ email, password }` | `{ token, user: { id, email, name, role } }` |
| POST | `/api/auth/register` | `{ email, password, name? }` | `{ token, user }` |
| POST | `/api/auth/logout` | — | `{}` |
| GET | `/api/auth/me` | — | `{ id, email, name, role }` |
| POST | `/api/auth/forgot-password` | `{ email }` | `{}` |
| POST | `/api/auth/reset-password` | `{ token, new_password }` | `{}` |
| POST | `/api/auth/refresh` | — | `{ token }` |

**Authentication:**  
All endpoints except `/login`, `/register`, `/forgot-password`, and `/reset-password` require `Authorization: Bearer <token>` header.

---

## Storage Keys

The library uses the following localStorage keys:

- `rustok-admin-session` — Full session object (JSON)
- `rustok-admin-token` — JWT token (string)
- `rustok-admin-tenant` — Tenant slug (string)
- `rustok-admin-user` — User object (JSON)

---

## Multi-Tenant Support

Pass tenant slug when signing in/up:

```rust
auth.sign_in(email, password, "acme-corp".to_string()).await?;
```

Tenant is stored in session and can be retrieved with:

```rust
let tenant = use_tenant();
```

---

## Error Handling

```rust
match auth.sign_in(email, password, tenant).await {
    Ok(_) => {
        // Success
    }
    Err(AuthError::InvalidCredentials) => {
        // Wrong email/password
    }
    Err(AuthError::Unauthorized) => {
        // Token expired or invalid
    }
    Err(AuthError::Network) => {
        // Network/parsing error
    }
    Err(AuthError::Http(status)) => {
        // Other HTTP error
    }
}
```

---

## License

MIT

---

## Contributing

This library is part of the RusToK project. See main repository for contribution guidelines.

---

**Version:** 0.1.0  
**Last Updated:** 2026-02-13  
**Status:** ✅ Production Ready
