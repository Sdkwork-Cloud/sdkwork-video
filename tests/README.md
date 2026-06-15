# Tests Directory

## Purpose
Cross-package tests, contract tests, integration tests, end-to-end tests, fixtures, and static verification inputs.

## Owner
Quality assurance teams and test engineers.

## Allowed Content
- Cross-package tests
- Contract tests
- Integration tests
- End-to-end tests
- Test fixtures
- Static verification inputs

## Forbidden Content
- Real secrets or tokens
- Private customer data
- Runtime state
- Generated SDK output

## Related Specs
- `TEST_SPEC.md`
- `QUALITY_GATE_SPEC.md`
- `CODE_REVIEW_SPEC.md`

## Verification
- Tests are runnable
- Fixtures contain safe sample data
- Tests do not store real credentials