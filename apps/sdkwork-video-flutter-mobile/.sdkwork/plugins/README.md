# SDKWork Video Flutter Mobile Plugins

## Purpose
Repository/application-local agent extensions and plugin bundles for SDKWork Video Flutter Mobile application.

## How to Add Plugins
1. Create a new directory under `.sdkwork/plugins/` with lowercase kebab-case name
2. Add `.codex-plugin/plugin.json` for installable plugins
3. Include `skills/`, `mcp/`, `apps/`, and `scripts/` directories as needed
4. Follow `SDKWORK_WORKSPACE_SPEC.md` for plugin structure rules

## Plugin Categories
- Flutter build and packaging plugins
- Dart SDK generation plugins
- iOS/Android platform adapter plugins
- Development workflow plugins

## Related Specs
- `SDKWORK_WORKSPACE_SPEC.md` (section 4)
- `AGENTS_SPEC.md`
- `SOUL.md`

## Rules
- Plugins must not vendor unrelated external toolchains
- Plugins must not store secrets or runtime data
- Plugins must call canonical commands defined by relevant specs