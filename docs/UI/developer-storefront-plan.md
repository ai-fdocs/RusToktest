# Developer Storefront Plan — Phase 1 (recovered)

## Принципы (сверху, как договорились)

- Мы **не клонируем** библиотеки целиком. Вместо этого делаем **минимальные адаптеры/обёртки** и закрываем пробелы **по мере работы** с админками/витриной.
- Приоритет — **готовые библиотеки и интеграции**; самопис — только если нет адекватного аналога.
- Любые отклонения фиксируем в UI‑документах и матрицах паритета.

См. базовые источники:
- [UI parity (admin + storefront)](./ui-parity.md)
- [Admin libraries parity](./admin-libraries-parity.md)
- [Admin reuse matrix](./admin-reuse-matrix.md)
- [Tech parity tracker](./tech-parity.md)
- [Storefront overview](./storefront.md)
- [Phase 2.1 — Users vertical slice](./phase2-users-vertical-slice.md)

---

## Phase 1 — чек‑лист (восстановлено по коду)

> Формат: 1) пункт → Leptos / Next

### Админки (Leptos + Next.js)

1. Базовый layout и навигационный shell админки.
   - [x] Leptos
   - [x] Next
2. Dashboard/главная админки.
   - [x] Leptos
   - [x] Next
3. Страницы аутентификации: login / register / reset.
   - [x] Leptos
   - [x] Next
4. Страница Security.
   - [x] Leptos
   - [x] Next
5. Страница Profile.
   - [x] Leptos
   - [x] Next
6. Users list с фильтрами/поиском и пагинацией (REST + GraphQL запросы).
   - [x] Leptos
   - [x] Next
7. User details (карточка пользователя).
   - [x] Leptos
   - [ ] Next
8. Auth‑guard (защита приватных маршрутов).
   - [x] Leptos
   - [x] Next
9. Базовые UI‑примитивы (PageHeader, кнопки, инпуты) в shadcn‑style.
   - [x] Leptos
   - [x] Next
10. i18n (RU/EN).
   - [x] Leptos
   - [x] Next

### Storefront (Leptos SSR + Next.js)

1. Landing‑shell (hero + CTA + основной layout).
   - [x] Leptos
   - [x] Next
2. Блоки контента (карточки/фичи/коллекции).
   - [x] Leptos
   - [x] Next
3. Блоки маркетинга/инфо (alert/статы/история бренда/подписка).
   - [x] Leptos
   - [x] Next
4. i18n / локализация витрины.
   - [x] Leptos
   - [x] Next
5. Tailwind‑стили и базовая тема (DaisyUI/shadcn‑style).
   - [x] Leptos
   - [x] Next
6. SSR‑сервер + отдача CSS‑бандла.
   - [x] Leptos
   - [ ] Next

---

## Phase 2.1 — Users vertical slice (только работы)

> Формат: 1) пункт → Leptos / Next

1. i18n foundation (ключевые неймспейсы и единые ключи).
   - [ ] Leptos
   - [ ] Next
2. Auth wiring (REST: login/me, хранение токена, guard).
   - [ ] Leptos
   - [ ] Next
3. Users list + pagination + filters (GraphQL users query).
   - [ ] Leptos
   - [ ] Next
4. Users detail view (GraphQL user query).
   - [ ] Leptos
   - [ ] Next
5. Users CRUD (create/update/disable + формы и ошибки).
   - [ ] Leptos
   - [ ] Next
6. Shared UI/UX (layout/nav, breadcrumbs, toasts, form patterns).
   - [ ] Leptos
   - [ ] Next

---

## Phase 3 — Admin Auth & User Security (только работы)

> Формат: 1) пункт → Leptos / Next

1. Admin auth middleware/guard (требовать auth на приватных маршрутах).
   - [ ] Leptos
   - [ ] Next
2. Login flow (REST: `POST /api/auth/login`).
   - [ ] Leptos
   - [ ] Next
3. Session bootstrap (REST: `GET /api/auth/me`).
   - [ ] Leptos
   - [ ] Next
4. Token storage + refresh strategy (cookie/localStorage).
   - [ ] Leptos
   - [ ] Next
5. Logout flow (очистка токена/сессии).
   - [ ] Leptos
   - [ ] Next
6. Password reset flow (request + confirm).
   - [ ] Leptos
   - [ ] Next
7. Security settings screen (пароли, сессии, 2FA placeholder).
   - [ ] Leptos
   - [ ] Next
8. RBAC checks for admin-only GraphQL/REST.
   - [ ] Leptos
   - [ ] Next
9. Error mapping для auth (errors.*).
   - [ ] Leptos
   - [ ] Next

---

## Phase 4 — Интеграция UI‑шаблона для админок (только работы)

> Формат: 1) пункт → Leptos / Next

1. Подготовка и аудит: цели, инвентаризация шаблона и текущих админок, UI контракт.
   - [ ] Leptos
   - [ ] Next
2. Карта соответствий (Template → RusToK): страницы, компоненты, токены.
   - [ ] Leptos
   - [ ] Next
3. Интеграция шаблона в Next.js админку: зависимости, layout/nav, страницы, i18n, API‑состояния.
   - [ ] Leptos
   - [ ] Next
4. Интеграция шаблона в Leptos админку: компоненты, layout/nav, страницы, i18n, API‑состояния.
   - [ ] Leptos
   - [ ] Next
5. Паритет и QA: визуальный паритет, поведение, доступность, производительность.
   - [ ] Leptos
   - [ ] Next
6. План внедрения/отката и DoD.
   - [ ] Leptos
   - [ ] Next
