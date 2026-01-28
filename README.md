# RusToK

## Русская версия

**RusToK** — модульная headless-платформа на Rust для построения CMS/Commerce-решений уровня enterprise. Проект ставит во главу угла стабильность, строгую типизацию, предсказуемые данные и модульность без «спагетти-плагинов».

### Ключевая идея

- **«The Tank Strategy»**: надежность и предсказуемость важнее хаотичной экосистемы плагинов.
- **Compile-time безопасность**: если компилируется — значит работает.
- **Headless-архитектура**: Storefront отделен от backend и общается только через GraphQL.

### Почему Rust

Rust выбран для того, чтобы сочетать **высокую производительность** и **строгую безопасность памяти** с удобной архитектурой уровня enterprise:

- **Zero-cost abstractions** и отсутствие GC помогают удерживать latency стабильным под нагрузкой.
- **Строгая типизация** снижает количество runtime-ошибок и упрощает поддержку сложных доменных моделей.
- **Concurrency-by-design** (Tokio) дает хороший масштабируемый IO-перформанс без тяжелых runtime.

### Сравнение с классическими CMS

| Критерий | RusToK | Классические CMS (PHP/монолитные) |
|---|---|---|
| Архитектура | Headless, модульная, GraphQL | Часто монолит, REST/HTML смешаны |
| Типизация | Строгая, compile-time | Динамическая/частично типизированная |
| Модульность | Модули как Rust-крейты | Плагины разного качества |
| Безопасность | Безопасная работа с памятью, меньше runtime-ошибок | Часто слабее контроль ошибок |
| Производительность | Высокая, стабильная под нагрузкой | Зависит от runtime и плагинов |

RusToK ориентирован на **предсказуемость и масштабируемость**, а не на мгновенное расширение через хаотичный набор плагинов.

### На чем построен проект

- **Rust 100%**, полностью типизированный backend.
- **Loco.rs** (Axum-основанный MVC), **Tokio** как async runtime.
- **PostgreSQL** со строгой реляционной схемой, **SeaORM** как ORM.
- **GraphQL** через async-graphql.
- **Leptos** для Admin UI (CSR) и Storefront (SSR).

### Структура проекта

```text
rustok/
├── apps/
│   ├── server/        # Backend (Loco.rs + GraphQL)
│   ├── admin/         # Admin UI (Leptos CSR)
│   └── storefront/    # Storefront (Leptos SSR)
├── crates/            # Модули и ядро
│   ├── rustok-core/
│   ├── rustok-commerce/
│   └── rustok-blog/
└── Cargo.toml         # Workspace
```

### Модули и расширяемость

Модули подключаются как Rust-крейты и регистрируются в сервере. Это обеспечивает строгую изоляцию, единые типы и прозрачные зависимости.

Пример модулей:
- **Commerce** — товары, заказы, категории.
- **Blog** — публикации и контент.

### Нагрузки и производительность

RusToK проектируется как headless-платформа с упором на масштабируемость:

- **Async-архитектура (Tokio + Axum)** позволяет эффективно обслуживать большое количество параллельных запросов.
- **Строгая схема PostgreSQL** дает предсказуемые запросы и возможность оптимизации на уровне БД.

#### Оценки нагрузки

> ⚠️ **Важно:** точные цифры по RPS и latency зависят от окружения, модели данных и бизнес-логики. В репозитории пока нет официальных бенчмарков.

Ожидаемая модель масштабирования:
- **Горизонтальное масштабирование** backend-сервиса (stateless + GraphQL).
- Оптимизация через кэширование и масштабирование PostgreSQL.

### Потребление памяти

> ⚠️ **Важно:** потребление памяти зависит от количества модулей, нагрузки, кэшей и размера данных.

Общие принципы потребления памяти:
- Rust не использует GC, поэтому пиковое потребление памяти обычно **предсказуемее**, чем в языках с GC.
- Основной вклад дают **активные запросы**, **ORM-кэш/соединения** и **In-memory события** (EventBus).

Для продакшена рекомендуется проводить нагрузочные тесты в своей среде.

### Быстрый старт

```bash
# Запуск БД
docker run -d --name rustok-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=rustok_dev \
  -p 5432:5432 \
  postgres:16

# Миграции и запуск сервера
cd apps/server
cargo loco db migrate
cargo loco start
```

### Статус

RusToK развивается как **enterprise headless CMS/Commerce**, с целью предоставить строгую архитектуру, модульность и стабильность уровня «танка».

---

## English version

**RusToK** is a modular headless Rust platform for building enterprise-grade CMS/Commerce solutions. The project prioritizes stability, strong typing, predictable data, and modularity without “spaghetti plugins.”

### Core idea

- **“The Tank Strategy”**: reliability and predictability over a chaotic plugin ecosystem.
- **Compile-time safety**: if it compiles, it works.
- **Headless architecture**: Storefront is separated from the backend and communicates exclusively via GraphQL.

### Why Rust

Rust combines **high performance** and **memory safety** with an architecture suitable for enterprise systems:

- **Zero-cost abstractions** and no GC help keep latency stable under load.
- **Strong typing** reduces runtime errors and simplifies complex domain models.
- **Concurrency-by-design** (Tokio) delivers scalable IO performance without heavy runtimes.

### Compared to classic CMS

| Criteria | RusToK | Classic CMS (PHP/monoliths) |
|---|---|---|
| Architecture | Headless, modular, GraphQL | Often monolithic, REST/HTML mixed |
| Typing | Strong, compile-time | Dynamic/partially typed |
| Modularity | Modules as Rust crates | Plugins of varying quality |
| Security | Memory safety, fewer runtime errors | Often weaker error control |
| Performance | High and stable under load | Varies by runtime/plugins |

RusToK is built for **predictability and scalability**, not for quick expansion via fragile plugins.

### Built on

- **Rust 100%** backend with strong typing.
- **Loco.rs** (Axum-based MVC) with **Tokio** as the async runtime.
- **PostgreSQL** with a strict relational schema and **SeaORM**.
- **GraphQL** via async-graphql.
- **Leptos** for Admin UI (CSR) and Storefront (SSR).

### Project structure

```text
rustok/
├── apps/
│   ├── server/        # Backend (Loco.rs + GraphQL)
│   ├── admin/         # Admin UI (Leptos CSR)
│   └── storefront/    # Storefront (Leptos SSR)
├── crates/            # Modules and core
│   ├── rustok-core/
│   ├── rustok-commerce/
│   └── rustok-blog/
└── Cargo.toml         # Workspace
```

### Modules and extensibility

Modules are Rust crates registered in the server. This keeps isolation strict, types shared, and dependencies explicit.

Example modules:
- **Commerce** — products, orders, categories.
- **Blog** — posts and content.

### Load and performance

RusToK is designed as a scalable headless platform:

- **Async architecture (Tokio + Axum)** handles many concurrent requests efficiently.
- **Strict PostgreSQL schema** enables predictable queries and DB-level optimization.

#### Load estimates

> ⚠️ **Note:** exact RPS and latency depend on environment, data model, and business logic. There are no official benchmarks in the repo yet.

Expected scaling model:
- **Horizontal scaling** of the stateless backend + GraphQL.
- Caching and PostgreSQL scaling as needed.

### Memory usage

> ⚠️ **Note:** memory consumption depends on enabled modules, workload, caches, and data sizes.

General memory behavior:
- Rust has no GC, so peak memory is often **more predictable** than GC-based languages.
- The main contributors are **active requests**, **ORM cache/connection pools**, and **in-memory events** (EventBus).

For production, run load tests in your own environment.

### Quickstart

```bash
# Start DB
docker run -d --name rustok-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=rustok_dev \
  -p 5432:5432 \
  postgres:16

# Migrate and start the server
cd apps/server
cargo loco db migrate
cargo loco start
```

### Status

RusToK is evolving as an **enterprise headless CMS/Commerce** focused on strong architecture, modularity, and “tank-level” stability.
