# leptos-auth

## Назначение

`crates/leptos-auth` — Leptos authentication library для RusToK, использующая **GraphQL** для всех операций.

## Архитектура

**Главное правило:** ✅ **Только GraphQL, никакого REST API!**

Эта библиотека предоставляет:
- Компоненты для защищённых маршрутов (`ProtectedRoute`, `GuestRoute`)
- Hooks для работы с аутентификацией (`use_auth`, `use_token`, `use_tenant`)
- GraphQL API для auth operations (`sign_in`, `sign_up`, `sign_out`)
- LocalStorage helpers для сохранения сессии

## Взаимодействие

- `apps/admin` — использует для аутентификации
- `apps/storefront` — использует для аутентификации
- `leptos-graphql` — низкоуровневый transport layer
- `apps/server` — GraphQL mutations/queries на backend

## Структура

```
src/
├── lib.rs          ← Public API, типы (AuthUser, AuthSession, AuthError)
├── api.rs          ← GraphQL mutations (sign_in, sign_up, sign_out)
├── context.rs      ← AuthProvider component, AuthContext
├── hooks.rs        ← use_auth(), use_token(), use_tenant(), etc.
├── storage.rs      ← LocalStorage helpers
└── components.rs   ← ProtectedRoute, GuestRoute, RequireAuth
```

## Использование

### 1. Обернуть приложение в AuthProvider

```rust
// apps/admin/src/app.rs
use leptos_auth::AuthProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AuthProvider>
            <Router>
                {/* routes */}
            </Router>
        </AuthProvider>
    }
}
```

### 2. Login page

```rust
use leptos::*;
use leptos_auth::api;

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    
    let login_action = create_action(|_| async move {
        match api::sign_in(
            email.get(),
            password.get(),
            "demo".to_string(), // tenant
        ).await {
            Ok((user, session)) => {
                // Success - AuthContext updated automatically
                navigate("/dashboard");
            }
            Err(e) => {
                // Handle error
            }
        }
    });
    
    view! {
        <form on:submit=|ev| {
            ev.prevent_default();
            login_action.dispatch(());
        }>
            <input type="email" value=email />
            <input type="password" value=password />
            <button type="submit">"Login"</button>
        </form>
    }
}
```

### 3. Protected routes

```rust
use leptos::*;
use leptos_router::*;
use leptos_auth::ProtectedRoute;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/login" view=Login />
                
                <ParentRoute path="" view=ProtectedRoute>
                    <Route path="/dashboard" view=Dashboard />
                    <Route path="/profile" view=Profile />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
```

### 4. Use auth hooks

```rust
use leptos::*;
use leptos_auth::{use_auth, use_token, use_tenant, use_current_user};

#[component]
pub fn Dashboard() -> impl IntoView {
    let auth = use_auth();
    let user = use_current_user();
    let token = use_token();
    let tenant = use_tenant();
    
    view! {
        <div>
            <p>"Welcome, " {move || user.get().map(|u| u.email)}</p>
            
            <button on:click=move |_| {
                spawn_local(async move {
                    let _ = auth.sign_out().await;
                });
            }>
                "Logout"
            </button>
        </div>
    }
}
```

### 5. Domain operations (using token)

```rust
use leptos::*;
use leptos_graphql::{execute, GraphqlRequest, GRAPHQL_ENDPOINT};
use leptos_auth::{use_token, use_tenant};

const GET_USERS_QUERY: &str = r#"
query GetUsers {
    users {
        items { id email name }
    }
}
"#;

#[component]
pub fn Users() -> impl IntoView {
    let token = use_token();
    let tenant = use_tenant();
    
    let users = create_resource(
        move || (token.get(), tenant.get()),
        |(token, tenant)| async move {
            let request = GraphqlRequest::new(GET_USERS_QUERY, None);
            execute(GRAPHQL_ENDPOINT, request, token, tenant).await
        },
    );
    
    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || users.get().map(|data| /* render */)}
        </Suspense>
    }
}
```

## GraphQL Mutations/Queries

### Authentication

```graphql
# Login
mutation SignIn($email: String!, $password: String!) {
    signIn(email: $email, password: $password) {
        token
        user { id email name }
    }
}

# Register
mutation SignUp($email: String!, $password: String!, $name: String) {
    signUp(email: $email, password: $password, name: $name) {
        token
        user { id email name }
    }
}

# Logout
mutation SignOut {
    signOut
}

# Current user
query CurrentUser {
    currentUser { id email name }
}

# Refresh token
mutation RefreshToken {
    refreshToken { token }
}

# Password reset
mutation ForgotPassword($email: String!) {
    forgotPassword(email: $email)
}

mutation ResetPassword($token: String!, $newPassword: String!) {
    resetPassword(token: $token, newPassword: $newPassword)
}
```

## API Functions

### `api::sign_in(email, password, tenant)`
Login через GraphQL mutation `signIn`.

**Returns:** `(AuthUser, AuthSession)`

### `api::sign_up(email, password, name, tenant)`
Register через GraphQL mutation `signUp`.

**Returns:** `(AuthUser, AuthSession)`

### `api::sign_out(token, tenant)`
Logout через GraphQL mutation `signOut`.

### `api::get_current_user(token, tenant)`
Get current user через GraphQL query `currentUser`.

**Returns:** `AuthUser`

### `api::refresh_token(token, tenant)`
Refresh JWT token через GraphQL mutation `refreshToken`.

**Returns:** `String` (new token)

### `api::forgot_password(email, tenant)`
Send password reset email.

### `api::reset_password(token, new_password, tenant)`
Reset password with token.

## Hooks

### `use_auth() -> AuthContext`
Get full auth context with methods.

### `use_current_user() -> Signal<Option<AuthUser>>`
Get current user (reactive).

### `use_token() -> Signal<Option<String>>`
Get JWT token (reactive).

### `use_tenant() -> Signal<Option<String>>`
Get tenant slug (reactive).

### `use_is_authenticated() -> Signal<bool>`
Check if user is authenticated (reactive).

### `use_is_loading() -> Signal<bool>`
Check if auth is loading (reactive).

### `use_session() -> Signal<Option<AuthSession>>`
Get full session (token + tenant).

## Components

### `<ProtectedRoute>`
Redirect to `/login` if not authenticated.

```rust
<ParentRoute path="" view=ProtectedRoute>
    <Route path="/dashboard" view=Dashboard />
</ParentRoute>
```

### `<GuestRoute>`
Redirect to `/dashboard` if already authenticated.

```rust
<Route path="/login" view=GuestRoute>
    <Login />
</Route>
```

### `<RequireAuth>`
Show fallback if not authenticated (inline).

```rust
<RequireAuth fallback=|| view! { <p>"Please login"</p> }>
    <SecretContent />
</RequireAuth>
```

## Types

### `AuthUser`
```rust
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}
```

### `AuthSession`
```rust
pub struct AuthSession {
    pub token: String,
    pub tenant: String,
}
```

### `AuthError`
```rust
pub enum AuthError {
    Unauthorized,
    InvalidCredentials,
    Network,
    Http(u16),
}
```

## Backend Requirements

Backend должен реализовать GraphQL mutations/queries в `apps/server/src/graphql/`:

- `mutation signIn(email, password) -> SignInPayload`
- `mutation signUp(email, password, name) -> SignUpPayload`
- `mutation signOut -> Boolean`
- `query currentUser -> User`
- `mutation refreshToken -> RefreshTokenPayload`
- `mutation forgotPassword(email) -> Boolean`
- `mutation resetPassword(token, newPassword) -> Boolean`

См. полную документацию: `/docs/UI/GRAPHQL_ARCHITECTURE.md`

## Dependencies

```toml
[dependencies]
leptos = { workspace = true }
leptos_router = { workspace = true }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
gloo-storage = { workspace = true }
thiserror = { workspace = true }
reqwest = { version = "0.13", default-features = false, features = ["json"] }
```

## Best Practices

1. **Всегда используйте константы для GraphQL queries**
   ```rust
   const SIGN_IN_MUTATION: &str = r#"..."#;
   ```

2. **Типизируйте ответы**
   ```rust
   #[derive(Deserialize)]
   struct SignInData { sign_in: SignInPayload }
   ```

3. **Обрабатывайте ошибки**
   ```rust
   match api::sign_in(...).await {
       Ok(_) => { /* success */ },
       Err(AuthError::InvalidCredentials) => { /* show error */ },
       Err(_) => { /* network error */ },
   }
   ```

4. **Используйте hooks для реактивности**
   ```rust
   let user = use_current_user();
   view! { <p>{move || user.get().map(|u| u.email)}</p> }
   ```

## Documentation

- Локальная: `./docs/` (пока нет)
- Общая: `/docs/UI/GRAPHQL_ARCHITECTURE.md`
- Backend GraphQL schema: `/apps/server/src/graphql/`

## Паспорт компонента

- **Роль:** Authentication library для Leptos apps (GraphQL-only)
- **Ответственность:** Auth state management, GraphQL auth operations, LocalStorage
- **Взаимодействует с:**
  - `leptos-graphql` (transport)
  - `apps/server` (GraphQL backend)
  - `apps/admin` (consumer)
  - `apps/storefront` (consumer)
- **Точки входа:** `src/lib.rs`
- **Документация:** `/docs/UI/GRAPHQL_ARCHITECTURE.md`

## Status

✅ **Реализовано** (GraphQL-only)

**Требуется на backend:**
- ⬜ Implement `mutation signIn`
- ⬜ Implement `mutation signUp`
- ⬜ Implement `mutation signOut`
- ⬜ Implement `query currentUser`
- ⬜ Implement `mutation refreshToken`
- ⬜ Implement `mutation forgotPassword`
- ⬜ Implement `mutation resetPassword`
