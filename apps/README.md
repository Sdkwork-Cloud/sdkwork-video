# Apps Directory

## Purpose
Independently runnable application roots, application surfaces, app shells, demos promoted to runnable apps, or deployable application compositions.

## Owner
Application maintainers and product teams.

## Allowed Content
- Application root directories
- Application surface configurations
- App shell implementations
- Deployable application compositions

## Forbidden Content
- Shared libraries (use `crates/` or `packages/`)
- Generated SDK output (use `sdks/`)
- API contracts (use `apis/`)
- Deployment descriptors (use `deployments/`)

## Related Specs
- `APPLICATION_SPEC.md`
- `APP_PC_ARCHITECTURE_SPEC.md`
- `APP_H5_ARCHITECTURE_SPEC.md`
- `FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md`
- `APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`

## Verification
- Each application root has `AGENTS.md` and `.sdkwork/`
- Application roots follow their architecture-specific standards
- Package naming follows SDKWork conventions