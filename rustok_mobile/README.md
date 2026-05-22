# RusTok Mobile Workspace

Initial Flutter workspace scaffold based on `docs/research/flutter.md`.

## Structure

- `apps/rustok_admin_mobile` — host Flutter app shell.
- `packages/app_core` — shared core primitives.
- `packages/app_ui_kit` — design tokens and presentational widgets.
- `packages/app_graphql` — GraphQL transport wiring.
- `packages/app_route_contracts` — typed route/query contracts.
- `packages/app_module_contracts` — interfaces for module-owned mobile packages.

## Next steps

1. Wire GraphQL HTTP/WS client with tenant/auth/locale headers.
2. Add module registry generation from RusTok module manifests.
3. Start first module package (`rustok_auth_mobile`).
