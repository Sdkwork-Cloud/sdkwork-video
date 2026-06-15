# Specs Directory

## Purpose
Local component/application specs that extend, but do not contradict, the canonical specs directory.

## Owner
Application maintainers.

## Allowed Content
- `component.spec.json` files
- Local spec extensions
- Architecture decision records

## Forbidden Content
- Specs that contradict root `sdkwork-specs/`
- Generated SDK output
- Runtime configuration

## Related Specs
- `COMPONENT_SPEC.md`
- `GOVERNANCE_SPEC.md`

## Rules
- Local specs may narrow root standards but must not contradict them.
- Each package/crate/service should have its own `component.spec.json` when it needs local governance.