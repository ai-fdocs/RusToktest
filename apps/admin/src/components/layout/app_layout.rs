// App Layout Component (Sidebar + Header + Content)
use leptos::prelude::*;
use leptos_router::components::Outlet;

use super::header::Header;
use super::sidebar::Sidebar;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="flex h-screen bg-gray-50">
            // Sidebar
            <Sidebar />

            // Main Content Area
            <div class="flex-1 flex flex-col overflow-hidden">
                // Header
                <Header />

                // Page Content
                <main class="flex-1 overflow-y-auto p-6">
                    <div class="max-w-7xl mx-auto">
                        <Outlet />
                    </div>
                </main>
            </div>
        </div>
    }
}
