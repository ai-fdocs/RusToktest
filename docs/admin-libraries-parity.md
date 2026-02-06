# Паритет библиотек админок (Next.js ↔ Leptos)

Этот документ фиксирует **известные** соответствия библиотек между админками и станет базой для унификации стека.

## Известные аналоги (подтверждено в коде/доках)

| Категория | Next.js admin | Leptos admin | Примечание |
| --- | --- | --- | --- |
| CSS/дизайн-токены | TailwindCSS | TailwindCSS | Используется в обеих админках. |
| CSS pipeline | PostCSS + Autoprefixer | PostCSS + Autoprefixer | Одинаковая цепочка сборки стилей. |
| UI контракт | shadcn/ui | shadcn-style components | В документации зафиксирован единый shadcn‑style подход для обеих админок. |

## Требуют поиска и подтверждения

- Формы/валидация (Next.js: react-hook-form + zod).
- Таблицы (Next.js: @tanstack/react-table).
- Data fetching (Next.js: @tanstack/react-query).
- i18n (Next.js: next-intl).
- State (Next.js: zustand).
