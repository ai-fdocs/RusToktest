# rustok-page-builder: runtime-контракт

`rustok-page-builder` — референсный FBA-модуль визуального билдера.

## Назначение

Модуль вводит самостоятельный capability-контур билдера до интеграции в `pages`.
Это позволяет закрепить FBA-first delivery и контрактную совместимость между host-реализациями.

## Ответственности

- owner контракта visual builder payload (`grapesjs_v1`) на модульном уровне;
- lifecycle-рамка для rollout/health/observability в терминах FBA;
- совместимость с consumer-модулями по contract-first интеграции.

## Точки входа

- `src/lib.rs` — runtime metadata и permission surface;
- `rustok-module.toml` — декларация slug/entry type/ui-classification.
