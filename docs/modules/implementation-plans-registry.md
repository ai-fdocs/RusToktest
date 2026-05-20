# Реестр implementation plans (crate-level)

Этот реестр — единая операционная точка для сопровождения implementation plans по crate-ам.
Используйте его как "single pane of glass": сначала обновляйте статус здесь, затем переходите в локальный план модуля.

## Области покрытия

Каждый implementation plan в crate должен включать два обязательных направления в одном документе:

- feature delivery (функциональные этапы),
- quality backlog (тесты, документация, DX и quality gates).

Отдельный второй план для quality **не нужен**: качество ведётся в том же `docs/implementation-plan.md` через отдельную секцию/чеклист.

## Как работать с реестром

1. Найдите запись, на которую указывает `next_plan_id` в `Cycle state`.
2. Откройте linked plan и выполните ограниченный по времени итерационный шаг (рекомендуется 30–60 минут или 1 PR).
3. Внутри итерации обязательно сделать оба шага:
   - синхронизация плана с фактическим кодом,
   - выполнение следующего незавершённого пункта плана.
4. Обновите:
   - локальный план (checkpoint-блок),
   - этот реестр (`status`, `progress`, `last_updated_at`, `last_checkpoint`, `next_action`, `blockers`).
5. Сдвиньте `next_plan_id` на следующую запись по кругу (даже если текущий план заблокирован или завершён).

## Статусы

- `not_started` — работа не начата.
- `in_progress` — есть активная итерация.
- `blocked` — есть внешний блокер, требуется разблокировка.
- `done` — план завершён, verification пройден, docs синхронизированы.
- `archived` — план закрыт/заменён другим документом.

## Шаблон checkpoint-блока для локальных планов

В начало каждого implementation plan добавляйте и поддерживайте блок:

```md
## Execution checkpoint

- Current phase:
- Last checkpoint:
- Next step:
- Open blockers:
- Hand-off notes for next agent:
- Last updated at (UTC):
```

## Cycle state

| Field | Value | Notes |
|---|---|---|
| `cycle_id` | `2026-Q2-round-robin-v1` | Идентификатор текущего цикла |
| `next_plan_id` | `example-rustok-product` | ID записи, которую должен взять следующий агент |
| `last_rotation_at` | `2026-05-20T00:00:00Z` | Когда указатель был сдвинут последний раз |
| `rotation_rule` | `strict_round_robin` | Всегда следующий план по списку, без пропусков |

## Global board

| Plan ID | Module / crate | Plan doc | Status | Progress | Owner | Last updated (UTC) | Last checkpoint | Next action | Blockers | Verification gate |
|---|---|---|---|---|---|---|---|---|---|---|
| `example-rustok-product` | `rustok-product` | `crates/rustok-product/docs/implementation-plan.md` | `in_progress` | `45%` | `agent:planner-1` | `2026-05-20T00:00:00Z` | Synced plan with current pricing code and closed one backlog item | Implement write-path SSR tests and close next P0 quality backlog item | No blocking issues | `cargo test -p rustok-product --lib && cargo clippy -p rustok-product -- -D warnings` |

> Удалите примерную строку после заполнения реальными crate-планами.

## Round-robin protocol (для агентов)

1. Взять `next_plan_id` из `Cycle state`.
2. Выполнить один осмысленный инкремент по плану (sync + execution).
3. Обновить checkpoint в локальном плане.
4. Обновить статус в этом реестре.
5. Вычислить следующую запись по таблице `Global board` и записать её в `next_plan_id`.
6. Если возник блокер — перевести запись в `blocked` и явно зафиксировать условие разблокировки.

## Definition of done для пунктов плана

Пункт плана можно пометить `done` только если одновременно:

1. Изменение присутствует в коде.
2. Пройден соответствующий verification gate.
3. Локальный `implementation-plan.md` обновлён под фактическое состояние.

## Weekly sweep

Раз в неделю отдельный агент/ответственный выполняет sweep:

- отмечает stale-элементы (`last_updated_at` старше 7 дней),
- поднимает приоритеты для `blocked` записей,
- формирует краткий список "next up" для нового круга.

## Hygiene: как чистить таблицу, если раздулась

Чтобы реестр оставался рабочим, а не превращался в лог-архив:

1. Держите в `Global board` только live-записи (`not_started`, `in_progress`, `blocked`, `done` за последние 14 дней).
2. Полностью завершённые старые записи переносите в `docs/modules/implementation-plans-registry.archive.md` (append-only).
3. `archived` записи не удаляйте без следа: переносите с датой и причиной архивирования.
4. Если у плана сменился путь/название — обновляйте текущую строку, а не создавайте дубль.
5. При каждом weekly sweep удаляйте пустые/дублированные строки и проверяйте уникальность `Plan ID`.
