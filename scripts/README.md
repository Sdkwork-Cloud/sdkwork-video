# Scripts Directory

## Purpose
Thin command entrypoints for build, verification, generation, migration, packaging, and release workflows.

## Owner
Developer experience teams and build engineers.

## Allowed Content
- Build scripts
- Verification scripts
- Generation scripts
- Migration scripts
- Packaging scripts
- Release scripts

## Forbidden Content
- Reusable logic (use `tools/`)
- Application runtime code
- Generated SDK output
- API contracts

## Related Specs
- `GITHUB_WORKFLOW_SPEC.md`
- `RELEASE_SPEC.md`
- `TEST_SPEC.md`

## Verification
- Scripts are thin entrypoints
- Scripts are documented
- Scripts do not contain secrets