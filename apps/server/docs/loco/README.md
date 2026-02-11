# Loco.rs docs index for RusToK

Этот документ — **навигационный индекс** по Loco-документации в репозитории.

## Canonical source (читать в первую очередь)

1. [Upstream Loco.rs snapshot (`./upstream/`)](./upstream/)
   - Это pinned-копия официальной документации Loco.rs.
   - Версия источника зафиксирована в [`./upstream/VERSION`](./upstream/VERSION).

> **Правило для AI-агентов и контрибьюторов:** при вопросах по Loco **сначала сверяться с `upstream/`**, и только потом с локальными заметками ниже.

## Repo-specific notes (только отличия RusToK от default Loco)

- Серверная реализация живёт в `apps/server` и может вводить проектные ограничения поверх дефолтных возможностей Loco.
- При проектировании изменений приоритет у реального кода и текущих модулей (`app.rs`, `controllers/`, `models/`, `migration/`).
- Краткие изменения локальных практик ведутся в [`changes.md`](./changes.md).

## Обновление upstream snapshot

```bash
scripts/docs/sync_loco_docs.sh
```

Скрипт обновляет документы из официального источника, фильтрует релевантные разделы и записывает метаданные ревизии в `upstream/VERSION`.
