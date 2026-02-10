# leptos-graphql

## Назначение
`crates/leptos-graphql` — модуль/приложение RusToK. Здесь находится его код и корневая документация.

## Взаимодействие
- apps/admin
- apps/storefront
- apps/server (GraphQL endpoint)

## Документация
- Локальная документация: `./docs/`
- Общая документация платформы: `/docs`

## Паспорт компонента
- **Роль в системе:** Leptos-утилиты для GraphQL-запросов и интеграции с backend.
- **Основные данные/ответственность:** бизнес-логика и API данного компонента; структура кода и документации в корне компонента.
- **Взаимодействует с:**
  - apps/admin
  - apps/storefront
  - apps/server (GraphQL endpoint)
- **Точки входа:**
  - `crates/leptos-graphql/src/lib.rs`
- **Локальная документация:** `./docs/`
- **Глобальная документация платформы:** `/docs/`



## Практический подход (Leptos-way)
- Этот crate — тонкий transport/utils слой поверх `reqwest` + GraphQL payload/response.
- Управление async-состоянием (`loading/error/data`) остается в `leptos::Resource`/actions в приложениях.
- Для строгой типизации запросов можно подключать `graphql-client` в приложениях и отправлять сгенерированные payload через этот слой.
