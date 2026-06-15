# SDKWork Video H5 Skills

## Purpose
Reusable agent/operator workflows for SDKWork Video H5 application development.

## How to Add Skills
1. Create a new directory under `.sdkwork/skills/` with lowercase kebab-case name
2. Add `SKILL.md` as the entrypoint
3. Include `references/`, `scripts/`, and `assets/` directories as needed
4. Follow `SDKWORK_WORKSPACE_SPEC.md` for skill structure rules

## Skill Categories
- H5 build and packaging workflows
- Capacitor iOS/Android build workflows
- TypeScript SDK generation and verification
- Appbase IAM integration checks
- Release and deployment readiness checks

## Related Specs
- `SDKWORK_WORKSPACE_SPEC.md` (section 3)
- `AGENTS_SPEC.md`
- `SOUL.md`

## Rules
- Skills must cite relevant root specs
- Skills must not store application source code or secrets
- Skills must not weaken root specs or bypass SDK generation standards