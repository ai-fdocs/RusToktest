# Паритет библиотек для фронтендов (Leptos-first)

Этот документ фиксирует обязательный стек библиотек для наших фронтендов и приоритет на переиспользование готовых решений вместо самописа.

## Контекст

- **Описание:** единый набор библиотек для `apps/admin` (CSR) и `apps/storefront` (SSR).
- **Приоритет:** сначала фронтенды (storefront + admin), затем расширение в смежные приложения.
- **Ссылки:** [UI документы](./) • [UI parity](./ui-parity.md) • [IU библиотеки](../../IU/README.md)

## Согласованный core-стек для старта админки

Ниже — актуальный перечень, который мы согласовали для старта без лишнего усложнения.

| Категория | Библиотека/модуль | Что делает у нас |
| --- | --- | --- |
| UI core | `leptos`, `leptos_router` | Основа UI, реактивность и маршрутизация страниц админки |
| API transport | `leptos-graphql` | Внутренний GraphQL transport-слой (headers, extensions, error mapping) |
| HTTP/serialization | `reqwest`, `serde`, `serde_json` | Вызовы API и сериализация payload'ов |
| Auth | `leptos-auth` | Auth-flow для админки (наша Leptos-альтернатива NextAuth-подходу) |
| I18n | `leptos_i18n` | Многоязычность интерфейса |
| Metadata/SEO | `leptos-next-metadata` | Управление metadata/head в Next-like стиле |
| Forms | `leptos-hook-form`, `leptos-zod` | Управление формами и валидация |
| Tables/listing | `leptos-struct-table`, `leptos-shadcn-pagination` | Таблицы и пагинация для админских списков (из shadcn-слоя используем именно pagination crate) |
| Browser/reactive utils | `leptos-use` | LocalStorage/hooks/observer и прочие browser utilities |
| Styling | `tailwind-rs` | Tailwind pipeline для Leptos-UI |
| DX/debug | `console_error_panic_hook`, `console_log`, `log` | Диагностика ошибок и логирование в браузере |

## Что считаем опциональным

- `graphql-client` — подключаем в приложении, когда реально включаем typed `.graphql` codegen flow.
- `leptos-query` — подключаем точечно, когда нужен кэш/стратегии refetch/stale beyond базовых `Resource`/actions.
- `leptos-shadcn-ui` как полный набор примитивов сейчас **не является обязательным core**; в согласованном baseline используем `leptos-shadcn-pagination` и локальные UI-компоненты.

## Обязательный набор библиотек (подключаем в фронтендах)

| Категория | Библиотека | Где используем | Примечание |
| --- | --- | --- | --- |
| UI core | `leptos`, `leptos_router` | admin + storefront | Базовый UI и роутинг |
| Auth | `leptos-auth` | admin + storefront | Встроенная интеграция auth-flow |
| GraphQL transport | `leptos-graphql` | admin + storefront | Клиентский transport слой к `/api/graphql` |
| Forms | `leptos-hook-form` | admin + storefront | Единое состояние форм |
| Validation | `leptos-zod` | admin + storefront | Маппинг и формат ошибок валидации |
| Tables | `leptos-struct-table` | admin + storefront | Табличный UI-слой для Leptos |
| Pagination | `leptos-shadcn-pagination` | admin + storefront | Пагинация в shadcn-style |
| Local state | `leptos-zustand` | admin + storefront | Store snapshots/updates |
| Reactive/browser utils | `leptos-use` | admin + storefront | Подписки/observer/storage/events/debounce |
| I18n | `leptos_i18n` | admin + storefront | Мультиязычность (RU/EN и далее) |
| Metadata/SEO | `leptos-next-metadata` | storefront (+ при необходимости admin) | Next-like модель метаданных для Leptos |
| Async data/query | `leptos-query` | admin + storefront | Кэш, stale/refetch, query lifecycle |
| Styling pipeline | `tailwind-rs` | admin + storefront | TailwindCSS pipeline и токены |

## Правило для разработки

1. Перед новым UI-функционалом проверяем этот список и используем библиотеку из него.
2. Самопис допускается только если библиотека отсутствует или не закрывает критичный кейс.
3. Если добавляется новая библиотека, обновляем этот документ и `Cargo.toml` фронтендов в одном PR.

This is an alpha version and requires clarification. Be careful, there may be errors in the text. So that no one thinks that this is an immutable rule.
