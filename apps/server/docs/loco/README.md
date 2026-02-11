# Loco.rs context pack (for contributors and AI agents)

Цель папки — дать **актуальный контекст по Loco.rs** рядом с сервером, чтобы не "додумывать" интеграции.

## ⚠️ Для AI-агентов: читать в первую очередь

Если вы меняете `apps/server/**`, сначала проверьте:

1. Этот файл (`apps/server/docs/loco/README.md`);
2. `apps/server/docs/loco/changes.md`;
3. `apps/server/docs/library-stack.md` (основные библиотеки сервера и роли);
4. `apps/server/docs/loco/upstream/VERSION` (актуальность snapshot);
5. Текущие паттерны в `apps/server/src/**` и `apps/server/migration/**`.

Короткое правило: **реальный код в `apps/server` важнее абстрактных рекомендаций из интернета**.

## Что это

Это не полная копия внешней документации, а практичный reference для этого репозитория:

- как устроен сервер в `apps/server`;
- какие паттерны считаются корректными в Loco-проектах;
- какие команды и точки входа использовать в RusToK.

## Рекомендуемый workflow для изменений в сервере

1. Начинать с чтения:
   - `apps/server/src/app.rs`
   - `apps/server/src/controllers/`
   - `apps/server/src/models/`
   - `apps/server/migration/`
2. Для изменений схемы БД:
   - создать migration через Loco-подход;
   - проверить, что migration подключена в `migration/src/lib.rs`;
   - обновить связанные модели/DTO/валидаторы.
3. Для новых endpoint'ов:
   - сначала контракты (request/response/errors),
   - затем контроллер,
   - затем интеграция с auth/permission,
   - затем документация в `docs/`.

## Команды (локально)

```bash
cd apps/server

# Применить миграции
cargo loco db migrate

# Запустить сервер
cargo loco start
```

## Что важно для AI-агентов

- Loco.rs уже используется как backend framework — не предлагать замену фреймворка для базовых задач.
- Для auth, permissions, migrations и контроллеров опираться на текущие паттерны проекта, а не абстрактные "универсальные" рецепты.
- Если в коде есть расхождения между общим guidance и реальной реализацией — приоритет у реального кода в `apps/server`.

## Как поддерживать "свежесть" этого контекста

- При изменении server-архитектуры обновлять этот файл в том же PR.
- При крупных изменениях Loco-слоя добавлять короткие заметки в `apps/server/docs/loco/changes.md`.

## Upstream snapshot freshness

`apps/server/docs/loco/upstream/VERSION` stores snapshot metadata for upstream Loco references.

- `make docs-check-loco` validates that metadata exists and enforces freshness policy:
  - `>30` days old: CI warning;
  - `>60` days old: CI failure.
- `make docs-sync-loco` refreshes snapshot metadata date before opening a PR.

## Как удалить Loco-документацию и автоматизацию (если временная мера больше не нужна)

Удаляйте это одним PR, чтобы не оставлять «битые» CI-проверки:

1. Удалить папку документации:
   - `apps/server/docs/loco/` (включая `upstream/VERSION`).
2. Удалить скрипт автоматизации:
   - `scripts/loco_upstream_snapshot.py`.
3. Удалить make-цели:
   - `docs-sync-loco` и `docs-check-loco` из `Makefile`.
4. Удалить CI-job:
   - `loco-docs-snapshot` из `.github/workflows/ci.yml`;
   - убрать его из `ci-success.needs` и из финального условия проверки.
5. Удалить пункт из PR-шаблона:
   - checkbox про актуальность `apps/server/docs/loco/upstream`.

Минимальная проверка после удаления:

```bash
cargo check --workspace --all-targets --all-features
```

и убедиться, что workflow CI проходит без `loco-docs-snapshot`.
