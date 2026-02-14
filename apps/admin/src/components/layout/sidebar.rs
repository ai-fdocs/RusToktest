// Sidebar Navigation Component
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="w-64 bg-white border-r border-gray-200 h-screen flex flex-col">
            // Logo & Brand
            <div class="p-6 border-b border-gray-200">
                <A href="/" class="flex items-center gap-2">
                    <div class="w-8 h-8 bg-gradient-to-br from-blue-600 to-blue-800 rounded-lg flex items-center justify-center">
                        <span class="text-white font-bold text-sm">"R"</span>
                    </div>
                    <div>
                        <h1 class="text-lg font-bold text-gray-900">"RusToK"</h1>
                        <p class="text-xs text-gray-500">"Admin Panel"</p>
                    </div>
                </A>
            </div>

            // Navigation
            <nav class="flex-1 p-4 space-y-1 overflow-y-auto">
                <NavSection title="Overview">
                    <NavLink href="/dashboard" icon="📊">
                        "Dashboard"
                    </NavLink>
                    <NavLink href="/analytics" icon="📈">
                        "Analytics"
                    </NavLink>
                </NavSection>

                <NavSection title="Content">
                    <NavLink href="/posts" icon="📝">
                        "Posts"
                    </NavLink>
                    <NavLink href="/pages" icon="📄">
                        "Pages"
                    </NavLink>
                    <NavLink href="/media" icon="🖼️">
                        "Media"
                    </NavLink>
                </NavSection>

                <NavSection title="Commerce">
                    <NavLink href="/products" icon="🛍️">
                        "Products"
                    </NavLink>
                    <NavLink href="/orders" icon="📦">
                        "Orders"
                    </NavLink>
                    <NavLink href="/customers" icon="👥">
                        "Customers"
                    </NavLink>
                </NavSection>

                <NavSection title="System">
                    <NavLink href="/users" icon="👤">
                        "Users"
                    </NavLink>
                    <NavLink href="/settings" icon="⚙️">
                        "Settings"
                    </NavLink>
                </NavSection>
            </nav>

            // Footer
            <div class="p-4 border-t border-gray-200">
                <div class="text-xs text-gray-500 text-center">
                    "v0.1.0"
                </div>
            </div>
        </aside>
    }
}

#[component]
fn NavSection(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="py-2">
            <div class="px-3 mb-2 text-xs font-semibold text-gray-500 uppercase tracking-wider">
                {title}
            </div>
            <div class="space-y-1">
                {children()}
            </div>
        </div>
    }
}

#[component]
fn NavLink(href: &'static str, icon: &'static str, children: Children) -> impl IntoView {
    view! {
        <A
            href=href
            class="flex items-center gap-3 px-3 py-2 text-sm font-medium text-gray-700 rounded-lg hover:bg-gray-100 hover:text-gray-900 transition-colors"
            active_class="bg-blue-50 text-blue-700 hover:bg-blue-100 hover:text-blue-800"
        >
            <span class="text-lg">{icon}</span>
            <span>{children()}</span>
        </A>
    }
}
