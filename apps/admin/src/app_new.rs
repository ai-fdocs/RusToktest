// New App Component with Updated Layout and Routing
use leptos::prelude::*;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::path;

use crate::components::layout::AppLayout;
use crate::components::protected_route::ProtectedRoute;
use crate::pages::{
    dashboard::Dashboard, login_new::LoginNew, not_found::NotFound, profile::Profile,
    register_new::RegisterNew, reset::ResetPassword, security::Security,
    user_details::UserDetails, users::Users,
};
use crate::providers::auth::provide_auth_context;
use crate::providers::auth_new::AuthProvider;
use crate::providers::locale::provide_locale_context;

#[component]
pub fn App() -> impl IntoView {
    provide_auth_context();
    provide_locale_context();

    view! {
        <AuthProvider>
            <Router>
                <main class="min-h-screen bg-slate-100 text-slate-900 font-sans">
                    <Routes fallback=|| view! { <NotFound /> }>
                        // Auth routes (no layout)
                        <Route path=path!("/login") view=LoginNew />
                        <Route path=path!("/register") view=RegisterNew />
                        <Route path=path!("/reset") view=ResetPassword />

                        // Protected routes (with AppLayout)
                        <ParentRoute path=path!("") view=ProtectedRoute>
                            <ParentRoute path=path!("") view=AppLayout>
                                <Route path=path!("/dashboard") view=Dashboard />
                                <Route path=path!("/profile") view=Profile />
                                <Route path=path!("/security") view=Security />
                                <Route path=path!("/users") view=Users />
                                <Route path=path!("/users/:id") view=UserDetails />
                                <Route path=path!("") view=Dashboard />
                            </ParentRoute>
                        </ParentRoute>

                        <Route path=path!("/*") view=NotFound />
                    </Routes>
                </main>
            </Router>
        </AuthProvider>
    }
}
