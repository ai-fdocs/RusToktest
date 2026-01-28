<div align="center">

# 🦀 RusToK

**The Tank Strategy CMS.**  
Enterprise modular headless platform built 100% in Rust.

[![CI](https://github.com/RustokCMS/RusToK/actions/workflows/ci.yml/badge.svg)](https://github.com/RustokCMS/RusToK/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)
[![Stack: Loco.rs](https://img.shields.io/badge/Stack-Loco.rs-orange)](https://loco.rs)
[![Frontend: Leptos](https://img.shields.io/badge/Frontend-Leptos-red)](https://leptos.dev)

</div>

---

## 📖 О проекте

**RusToK** — попытка принести **строгую типизацию**, **memory safety** и **производительность** Rust в мир E-commerce и CMS, где десятилетиями правили PHP (WordPress, Magento) и JavaScript (Strapi). Мы строим **«Танк»**:

- если это компилируется — это работает;
- никаких `undefined is not a function` в продакшене;
- никакого «плагинного хаоса», который ломает базу данных при апдейтах;
- полная изоляция бизнес-логики от UI.

Проект построен на базе **Loco.rs** (Rust on Rails) и **Leptos**, обеспечивая единый язык (Rust) на бэкенде, фронтенде и в админ-панели.

---

## ⚔️ Сравнение с гигантами

Почему RusToK? Потому что динамическая типизация в enterprise — это бомба замедленного действия.

| Характеристика | 🐘 WordPress / Magento | 🛍️ Shopify / SaaS | 🦀 RusToK |
| :--- | :--- | :--- | :--- |
| **Язык** | PHP (динамическая типизация) | Закрытые платформы | **Rust** (строгая статическая) |
| **Надежность** | Runtime ошибки | Высокая, но код не ваш | **Compile-time гарантии** |
| **Производительность** | Требует тяжёлого кэширования | Зависит от тарифа | **Native binary** |
| **Архитектура** | Плагины патчат ядро | App Store и API лимиты | **Модульный монолит** |
| **Кастомизация** | Высокая, но хрупкая | Ограниченная | **Полный контроль кода** |
| **Developer Experience** | Устаревший DX | Проприетарный | **Loco.rs (Rails-like DX)** |

---

## 🏗️ Архитектура: The Tank Strategy

Rust Everywhere. Никакого переключения контекста между JS и Backend.

### Технологический стек

- **Backend:** [Loco.rs](https://loco.rs) (на базе Axum) — MVC-фреймворк уровня Rails.
- **Database:** PostgreSQL + [SeaORM](https://www.sea-ql.org/SeaORM/) (async & typed).
- **Admin Panel:** [Leptos](https://leptos.dev) (CSR) — SPA на WASM.
- **Storefront:** [Leptos](https://leptos.dev) (SSR) — Server-Side Rendering для SEO и скорости.
- **API:** GraphQL (async-graphql).

### Структура монорепозитория

```text
rustok/
├── apps/                 # Исполняемые приложения
│   ├── server/           # 🧠 Backend API (Loco.rs)
│   ├── admin/            # ⚙️ Admin Panel (Leptos CSR / WASM)
│   └── storefront/       # 🛍️ Public Store (Leptos SSR)
│
└── crates/               # Переиспользуемая бизнес-логика
    ├── rustok-core/      # Ядро (Auth, Tenants, Base traits)
    ├── rustok-commerce/  # E-commerce модуль (Products, Cart)
    └── rustok-blog/      # CMS модуль (Posts, Pages)
```

Ключевая особенность: Admin — это **schema-driven** приложение. Бэкенд отдаёт метаданные (типы полей), а админка их отрисовывает. Бэкенд никогда не импортирует код фронтенда.

---

## 🚀 Быстрый старт

### Требования

- Rust (stable)
- PostgreSQL
- Docker (опционально)

### Запуск

```bash
# База данных
docker run -d --name rustok-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=rustok_dev \
  -p 5432:5432 \
  postgres:16

# Миграции и запуск
cd apps/server
cargo loco db migrate
cargo loco start
```

---

## 🧪 Development

```bash
# Тесты
cargo test --workspace

# Форматирование
cargo fmt --all

# Линт
cargo clippy --workspace -- -D warnings
```

---

## 🤝 Contributing

Мы приветствуем вклад в развитие проекта. Убедитесь, что код проходит CI (fmt, clippy, tests).

1. Fork
2. Create feature branch (`git checkout -b feature/my-feature`)
3. Commit (`git commit -m "Add some feature"`)
4. Push (`git push origin feature/my-feature`)
5. Open Pull Request

---

## 📄 License

Проект распространяется по лицензии **AGPL-3.0**. Если вы предоставляете сервис на базе RusToK, изменения должны быть открыты. Подробнее в файле `LICENSE`.

Built with ❤️ and 🦀 by the RusToK team.
