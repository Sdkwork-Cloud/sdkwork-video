# Configs Directory

## Purpose
Source-controlled safe config templates, profile examples, config schemas, and non-secret runtime defaults.

## Owner
Platform teams and configuration maintainers.

## Allowed Content
- Config templates
- Profile examples
- Config schemas
- Non-secret runtime defaults

## Forbidden Content
- Live secrets
- Private keys
- Local override files
- Runtime user config
- Database credentials

## Related Specs
- `CONFIG_SPEC.md`
- `ENVIRONMENT_SPEC.md`
- `RUNTIME_DIRECTORY_SPEC.md`

## Verification
- Config files contain no secrets
- Templates are safe for source control
- Profiles cover dev/test/staging/prod