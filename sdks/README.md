# SDKs Directory

## Purpose
SDK family workspaces, SDK generation manifests, authority OpenAPI materialization outputs, derived `sdkgen` inputs, generated SDK language workspaces, and SDK component specs.

## Owner
SDK developers and API integration teams.

## Allowed Content
- SDK family directories
- SDK generation manifests
- OpenAPI materialization outputs
- Generated SDK language workspaces
- SDK component specs

## Forbidden Content
- Authored API contracts (use `apis/`)
- Implementation code
- Runtime configuration
- Secrets or credentials

## Related Specs
- `SDK_SPEC.md`
- `SDK_WORKSPACE_GENERATION_SPEC.md`
- `RPC_SDK_WORKSPACE_SPEC.md`

## Verification
- SDK families follow naming conventions
- Generated output is not hand-edited
- SDK control-plane reports are generator-owned