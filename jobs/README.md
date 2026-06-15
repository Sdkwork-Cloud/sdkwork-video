# Jobs Directory

## Purpose
Job definitions, schedules, queue bindings, batch descriptors, maintenance runbooks, and non-Rust job packages.

## Owner
Operations teams and backend developers.

## Allowed Content
- Job schedules and cron definitions
- Queue consumer bindings
- Batch job descriptors
- Maintenance runbooks
- Non-Rust job packages

## Forbidden Content
- Rust worker implementations (use `crates/`)
- API contracts (use `apis/`)
- Generated SDK output (use `sdks/`)
- Frontend application code

## Related Specs
- `EVENT_SPEC.md`
- `DEPLOYMENT_SPEC.md`
- `OBSERVABILITY_SPEC.md`

## Verification
- Job definitions are documented
- Schedules are valid cron expressions
- Queue bindings are properly configured