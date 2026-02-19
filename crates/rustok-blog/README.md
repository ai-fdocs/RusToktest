# rustok-blog

## Назначение
`rustok-blog` — модуль блогового функционала платформы RusToK, построенный поверх контентного ядра.

## Что делает
- Управляет постами блога (создание, редактирование, публикация, архивирование)
- Поддерживает комментарии с модерацией
- Работает с категориями и тегами
- Интегрируется с системой индексации для поиска
- Публикует доменные события для синхронизации

## Архитектура

### Wrapper Module
Blog — это "wrapper" модуль, что означает:
- **Нет собственных таблиц** — использует таблицы `rustok-content`
- **Добавляет бизнес-логику** — валидация, workflow, специфичные для блога правила
- **Типобезопасная state machine** — управление статусами постов

### State Machine

```
  ┌───────┐
  │ Draft │──────────────────┐
  └───┬───┘                  │
      │ publish()            │ archive()
      ↓                      │
  ┌───────────┐              │
  │ Published │──────────────┤
  └─────┬─────┘              │
        │ unpublish()        │ archive()
        │                    ↓
        └─────────→   ┌──────────┐
                      │ Archived │
                      └──────────┘
                         │ restore()
                         ↓
                      ┌───────┐
                      │ Draft │
                      └───────┘
```

### Ключевые компоненты

| Файл | Назначение |
|------|------------|
| `lib.rs` | Точка входа, экспорт API, определение модуля |
| `error.rs` | Обработка ошибок с RichError |
| `state_machine.rs` | Типобезопасная машина состояний |
| `services/post.rs` | Бизнес-логика постов |
| `dto/` | Data Transfer Objects для API |

## Использование

### Создание поста

```rust
use rustok_blog::{PostService, CreatePostInput};
use rustok_core::SecurityContext;

let service = PostService::new(db, event_bus);

let input = CreatePostInput {
    locale: "ru".to_string(),
    title: "Мой первый пост".to_string(),
    body: "Содержимое поста...".to_string(),
    excerpt: Some("Краткое описание".to_string()),
    slug: Some("my-first-post".to_string()),
    publish: false, // Создать как черновик
    tags: vec!["rust".to_string(), "tutorial".to_string()],
    category_id: None,
    metadata: None,
};

let post_id = service.create_post(tenant_id, security, input).await?;
```

### Публикация поста

```rust
service.publish_post(post_id, tenant_id, security).await?;
```

### Получение постов по тегу

```rust
let posts = service.get_posts_by_tag(tenant_id, "rust".to_string(), 1, 10).await?;
```

## Взаимодействие

| Модуль | Тип взаимодействия |
|--------|-------------------|
| `rustok-core` | События, типы ошибок, permissions |
| `rustok-content` | Хранение данных (nodes, bodies, translations) |
| `rustok-outbox` | TransactionalEventBus для надёжной доставки событий |
| `rustok-index` | Подписка на события для индексации |

## Permissions

Модуль определяет следующие permissions:

### Posts
- `posts:create` — создание постов
- `posts:read` — чтение постов
- `posts:update` — редактирование постов
- `posts:delete` — удаление постов
- `posts:list` — список постов
- `posts:publish` — публикация постов

### Comments
- `comments:create` — создание комментариев
- `comments:read` — чтение комментариев
- `comments:update` — редактирование комментариев
- `comments:delete` — удаление комментариев
- `comments:list` — список комментариев
- `comments:moderate` — модерация комментариев

### Categories & Tags
- `categories:*` — управление категориями
- `tags:*` — управление тегами

## Обработка ошибок

Модуль использует `RichError` для детальной информации об ошибках:

```rust
pub enum BlogError {
    PostNotFound(Uuid),
    CommentNotFound(Uuid),
    DuplicateSlug { slug: String, locale: String },
    CannotDeletePublished,
    Validation(String),
    // ...
}
```

## Тестирование

```bash
# Unit тесты
cargo test -p rustok-blog

# С property-based тестами
cargo test -p rustok-blog --features proptest

# Интеграционные тесты (требуется БД)
cargo test -p rustok-blog -- --ignored
```

## Кому нужен
- **Admin UI** — управление постами, комментариями, категориями
- **Storefront** — отображение блога, ленты, архивы
- **API consumers** — RSS, интеграции с соцсетями

## Документация
- Локальная документация: `./docs/`
- Общая документация платформы: `/docs`

## Паспорт компонента
- **Роль в системе:** Доменный модуль блога
- **Основные данные:** бизнес-логика постов, комментариев, категорий, тегов
- **Взаимодействует с:**
  - `crates/rustok-core` (events, errors, permissions)
  - `crates/rustok-content` (storage)
  - `crates/rustok-outbox` (TransactionalEventBus)
  - `crates/rustok-index` (search indexing)
- **Точки входа:** `crates/rustok-blog/src/lib.rs`
- **Статус:** ✅ Production Ready
