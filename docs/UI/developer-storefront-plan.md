# Developer Storefront Plan — Phase 1 (recovered)

## Принципы (сверху, как договорились)

- Мы **не клонируем** библиотеки целиком. Вместо этого делаем **минимальные адаптеры/обёртки** и закрываем пробелы **по мере работы** с админками/витриной.
- Приоритет — **готовые библиотеки и интеграции**; самопис — только если нет адекватного аналога.
- Любые отклонения фиксируем в UI‑документах и матрицах паритета.

См. базовые источники:
- [UI parity (admin + storefront)](./ui-parity.md)
- [Admin libraries parity](./admin-libraries-parity.md)
- [Admin template integration plan](./admin-template-integration-plan.md)
- [Admin auth phase 3 scope](./admin-auth-phase3.md)
- [Admin Phase 3 architecture](./admin-phase3-architecture.md)
- [Admin Phase 3 gap analysis](./admin-phase3-gap-analysis.md)
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
