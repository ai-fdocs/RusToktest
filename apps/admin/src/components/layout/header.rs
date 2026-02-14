// Header Component with User Menu
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::components::features::auth::user_menu::UserMenu;
use crate::providers::locale::translate;

#[component]
pub fn Header() -> impl IntoView {
    // Get current page title from route (можно улучшить через context)
    let page_title = signal("Dashboard".to_string());

    view! {
        <header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-6">
            // Left: Page Title + Breadcrumbs
            <div class="flex items-center gap-4">
                <h2 class="text-lg font-semibold text-gray-900">
                    {move || page_title.0.get()}
                </h2>
            </div>

            // Right: Search + Notifications + User Menu
            <div class="flex items-center gap-4">
                // Search
                <div class="relative">
                    <input
                        type="search"
                        placeholder="Search..."
                        class="w-64 px-4 py-2 text-sm border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    />
                    <span class="absolute right-3 top-2.5 text-gray-400">
                        "🔍"
                    </span>
                </div>

                // Notifications
                <button class="relative p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors">
                    <span class="text-xl">"🔔"</span>
                    // Notification badge
                    <span class="absolute top-1 right-1 w-2 h-2 bg-red-500 rounded-full"></span>
                </button>

                // User Menu
                <UserMenu />
            </div>
        </header>
    }
}
