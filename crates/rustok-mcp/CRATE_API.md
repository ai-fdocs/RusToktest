# rustok-mcp / CRATE_API

## Публичные модули
`server`, `tools`.

## Основные публичные типы и сигнатуры
- `pub async fn serve_stdio(config: McpServerConfig) -> Result<...>`
- `pub struct McpServerConfig`
- `pub struct RusToKMcpServer`
- Публичные MCP tools/resources из `tools::*`.

## События
- Публикует: N/A (RPC/MCP адаптер).
- Потребляет: команды/запросы MCP клиента.

## Зависимости от других rustok-крейтов
- `rustok-core`

## Частые ошибки ИИ
- Упоминание несуществующего `apps/mcp` — MCP сервер реализован как встроенный binary `rustok-mcp-server` внутри crate `rustok-mcp`.
- Нарушает формат MCP tool/result payload.
