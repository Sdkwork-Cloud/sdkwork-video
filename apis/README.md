# APIs Directory

## Purpose
Author-owned API contracts and API source inputs for all API kinds, including HTTP OpenAPI surfaces, RPC/proto contracts, async/event API manifests, API examples, API changelogs, and API validation inputs.

## Owner
Application maintainers and API contract authors.

## Allowed Content
- OpenAPI specifications
- Proto files for RPC services
- AsyncAPI/event manifests
- API examples and samples
- API changelogs
- API validation fixtures

## Forbidden Content
- Generated SDK output
- Implementation code
- Runtime configuration
- Secrets or credentials

## Related Specs
- `API_SPEC.md`
- `RPC_SPEC.md`
- `SDK_SPEC.md`
- `SDK_WORKSPACE_GENERATION_SPEC.md`

## Verification
- API contracts follow OpenAPI 3.1.2 stable profile
- Proto files follow protobuf style guide
- No generated SDK transport output in this directory