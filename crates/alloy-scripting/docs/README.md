# alloy-scripting docs

Документация модуля `crates/alloy-scripting`.

## Содержание

- [implementation-plan.md](./implementation-plan.md) — архитектура, компоненты, flow выполнения, future improvements

## Краткий обзор

`alloy-scripting` — скриптовый движок на базе Rhai для пользовательской автоматизации.

### Основные возможности

1. **Event hooks** — скрипты срабатывают на события сущностей (before_create, after_update, on_commit)
2. **Cron scheduler** — scheduled выполнение по расписанию
3. **API triggers** — скрипты как HTTP endpoints
4. **Manual execution** — ручной запуск через API

### Безопасность

- Resource limits (max_operations, timeout, call_depth)
- Auto-disable после 3 ошибок подряд
- Sandboxed execution (no FS/network access)

### Интеграция

Модуль предоставляет:
- `ScriptableEntity` trait для интеграции с доменными сущностями
- `HookExecutor` для удобного вызова hooks из сервисов
- `ScriptOrchestrator` для координации выполнения

См. [implementation-plan.md](./implementation-plan.md) для деталей.
