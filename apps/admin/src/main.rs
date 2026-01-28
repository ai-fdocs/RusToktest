use leptos::*;
use leptos_router::{Route, Router, Routes};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="app-shell">
                <Style />
                <Routes>
                    <Route path="" view=LoginPage />
                    <Route path="/login" view=LoginPage />
                    <Route path="/dashboard" view=DashboardPage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Style() -> impl IntoView {
    view! {
        <style>
            ":root {\n"
            "  color-scheme: light;\n"
            "  font-family: 'Inter', system-ui, -apple-system, sans-serif;\n"
            "  background-color: #f5f6fb;\n"
            "}\n"
            "* { box-sizing: border-box; }\n"
            "body { margin: 0; }\n"
            ".app-shell { min-height: 100vh; color: #0f172a; }\n"
            ".auth-grid { display: grid; grid-template-columns: 1.2fr 1fr; min-height: 100vh; }\n"
            ".auth-visual { padding: 72px; background: radial-gradient(circle at top left, #1e3a8a, #0f172a); color: #fff; display: flex; flex-direction: column; justify-content: center; gap: 24px; }\n"
            ".auth-visual h1 { font-size: 2.5rem; margin: 0; }\n"
            ".auth-visual p { margin: 0; font-size: 1.05rem; opacity: 0.85; }\n"
            ".auth-form { padding: 72px 80px; display: flex; flex-direction: column; justify-content: center; gap: 28px; background: #f8fafc; }\n"
            ".auth-card { background: #fff; border-radius: 24px; padding: 32px; box-shadow: 0 24px 60px rgba(15, 23, 42, 0.12); display: flex; flex-direction: column; gap: 20px; }\n"
            ".auth-card h2 { margin: 0; font-size: 1.75rem; }\n"
            ".auth-card p { margin: 0; color: #64748b; }\n"
            ".input-group { display: flex; flex-direction: column; gap: 8px; }\n"
            ".input-group label { font-size: 0.9rem; color: #475569; }\n"
            ".input-group input { padding: 12px 16px; border-radius: 12px; border: 1px solid #e2e8f0; font-size: 0.95rem; }\n"
            ".primary-button { background: #2563eb; color: #fff; border: none; padding: 12px 18px; border-radius: 12px; font-weight: 600; cursor: pointer; }\n"
            ".secondary-link { color: #2563eb; text-decoration: none; font-size: 0.9rem; }\n"
            ".dashboard { padding: 32px 40px 56px; }\n"
            ".dashboard-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 32px; }\n"
            ".dashboard-header h1 { margin: 0; font-size: 2rem; }\n"
            ".badge { background: #e2e8f0; padding: 6px 12px; border-radius: 999px; font-size: 0.85rem; color: #475569; }\n"
            ".stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 20px; margin-bottom: 32px; }\n"
            ".stat-card { background: #fff; padding: 20px; border-radius: 18px; box-shadow: 0 16px 30px rgba(15, 23, 42, 0.08); }\n"
            ".stat-card h3 { margin: 0 0 8px; font-size: 0.95rem; color: #64748b; }\n"
            ".stat-card strong { font-size: 1.5rem; }\n"
            ".dashboard-panels { display: grid; grid-template-columns: 1.4fr 1fr; gap: 24px; }\n"
            ".panel { background: #fff; border-radius: 20px; padding: 24px; box-shadow: 0 18px 36px rgba(15, 23, 42, 0.08); }\n"
            ".panel h4 { margin: 0 0 16px; }\n"
            ".activity-item { display: flex; justify-content: space-between; align-items: center; padding: 12px 0; border-bottom: 1px solid #e2e8f0; }\n"
            ".activity-item:last-child { border-bottom: none; }\n"
            ".quick-actions { display: grid; gap: 12px; }\n"
            ".quick-actions button { background: #f1f5f9; border: none; padding: 12px 16px; border-radius: 12px; text-align: left; font-weight: 600; }\n"
            "@media (max-width: 960px) {\n"
            "  .auth-grid { grid-template-columns: 1fr; }\n"
            "  .auth-form { padding: 48px 32px; }\n"
            "  .auth-visual { padding: 48px 32px; }\n"
            "  .dashboard-panels { grid-template-columns: 1fr; }\n"
            "}\n"
        </style>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    view! {
        <section class="auth-grid">
            <aside class="auth-visual">
                <span class="badge">"Admin Foundation"</span>
                <h1>"RusToK Control Center"</h1>
                <p>
                    "Управляйте тенантами, модулями и контентом в одном месте. "
                    "Настраиваемый доступ, быстрые действия и прозрачная аналитика."
                </p>
                <div>
                    <p><strong>"Входит в v1.0"</strong></p>
                    <p>"Логин, роли, графики активности и контроль модулей."</p>
                </div>
            </aside>
            <div class="auth-form">
                <div class="auth-card">
                    <div>
                        <h2>"Вход в админ-панель"</h2>
                        <p>"Введите рабочие данные для доступа к панели управления."</p>
                    </div>
                    <div class="input-group">
                        <label for="email">"Email"</label>
                        <input id="email" type="email" placeholder="admin@rustok.io" />
                    </div>
                    <div class="input-group">
                        <label for="password">"Пароль"</label>
                        <input id="password" type="password" placeholder="••••••••" />
                    </div>
                    <button class="primary-button" type="button">
                        "Продолжить"
                    </button>
                    <a class="secondary-link" href="/dashboard">
                        "Перейти в демонстрационный дашборд →"
                    </a>
                </div>
                <p style="margin:0; color:#64748b;">
                    "Нужен доступ? Напишите администратору безопасности для активации." 
                </p>
            </div>
        </section>
    }
}

#[component]
fn DashboardPage() -> impl IntoView {
    let stats = [
        ("Активные тенанты", "28", "+3 за неделю"),
        ("Модули в работе", "12", "Commerce, Blog, Tickets"),
        ("Время отклика API", "128ms", "−14% за 7 дней"),
        ("Задач в очереди", "7", "2 критичных"),
    ];

    let activity = [
        ("Новый тенант", "Nordic Supply", "2 минуты назад"),
        ("Модуль", "Commerce обновлён до v1.0.3", "20 минут назад"),
        ("Безопасность", "Обновлены роли редакторов", "1 час назад"),
        ("Контент", "Запущена публикация промо-страницы", "Сегодня"),
    ];

    view! {
        <section class="dashboard">
            <header class="dashboard-header">
                <div>
                    <span class="badge">"Dashboard"</span>
                    <h1>"Добро пожаловать, Админ"</h1>
                    <p style="margin:8px 0 0; color:#64748b;">
                        "Сводка системы RusToK: ключевые метрики и быстрый доступ к модулям." 
                    </p>
                </div>
                <button class="primary-button" type="button">"Создать тенант"</button>
            </header>

            <div class="stats-grid">
                {stats
                    .iter()
                    .map(|(title, value, hint)| {
                        view! {
                            <div class="stat-card">
                                <h3>{*title}</h3>
                                <strong>{*value}</strong>
                                <p style="margin:8px 0 0; color:#94a3b8;">{*hint}</p>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>

            <div class="dashboard-panels">
                <div class="panel">
                    <h4>"Последняя активность"</h4>
                    {activity
                        .iter()
                        .map(|(title, detail, time)| {
                            view! {
                                <div class="activity-item">
                                    <div>
                                        <strong>{*title}</strong>
                                        <p style="margin:4px 0 0; color:#64748b;">{*detail}</p>
                                    </div>
                                    <span class="badge">{*time}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
                <div class="panel">
                    <h4>"Быстрые действия"</h4>
                    <div class="quick-actions">
                        <button type="button">"Запустить аудит безопасности"</button>
                        <button type="button">"Открыть список модулей"</button>
                        <button type="button">"Проверить метрики API"</button>
                        <button type="button">"Сформировать отчёт по ролям"</button>
                    </div>
                </div>
            </div>
        </section>
    }
}

fn main() {
    mount_to_body(App);
}
