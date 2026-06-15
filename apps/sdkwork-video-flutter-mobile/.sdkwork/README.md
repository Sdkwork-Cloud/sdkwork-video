# SDKWork Video Flutter Mobile Workspace

## Purpose
Local knowledge and extension workspace for SDKWork Video Flutter Mobile application development.

## Owner
SDKWork Video Flutter Mobile application team.

## Authoritative Directories
- `.sdkwork/skills/` - Reusable agent/operator workflows
- `.sdkwork/plugins/` - Repository/application-local agent extensions

## Related Specs
- `SDKWORK_WORKSPACE_SPEC.md`
- `AGENTS_SPEC.md`
- `SOUL.md`

## Rules
- This directory is source workspace metadata, not runtime state.
- Skills store reusable workflows, not application source code.
- Plugins store agent extensions, not runtime plugins.
- Local-only state belongs in `.sdkwork/local/`, `.sdkwork/tmp/`, `.sdkwork/cache/`, or `.sdkwork/secrets/` (all ignored).