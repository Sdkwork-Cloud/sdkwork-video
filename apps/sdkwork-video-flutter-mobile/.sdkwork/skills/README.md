# SDKWork Video Flutter Mobile Skills

## Purpose
Reusable agent/operator workflows for SDKWork Video Flutter Mobile application development.

## How to Add Skills
1. Create a new directory under `.sdkwork/skills/` with lowercase kebab-case name
2. Add `SKILL.md` as the entrypoint
3. Include `references/`, `scripts/`, and `assets/` directories as needed
4. Follow `SDKWORK_WORKSPACE_SPEC.md` for skill structure rules

## Skill Categories
- Flutter build and packaging workflows
- Dart SDK generation and verification
- iOS/Android platform adapter checks
- Appbase Flutter IAM integration checks
- Release and deployment readiness checks

## Related Specs
- `SDKWORK_WORKSPACE_SPEC.md` (section 3)
- `AGENTS_SPEC.md`
- `SOUL.md`

## Rules
- Skills must cite relevant root specs
- Skills must not store application source code or secrets
- Skills must not weaken root specs or bypass SDK generation standards