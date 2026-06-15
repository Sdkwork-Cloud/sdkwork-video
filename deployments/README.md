# Deployments Directory

## Purpose
Deployment descriptors, environment topology, packaging handoff files, infrastructure examples, and release deployment documentation.

## Owner
DevOps teams and deployment engineers.

## Allowed Content
- Deployment descriptors
- Environment topology
- Packaging handoff files
- Infrastructure examples
- Deployment runbooks

## Forbidden Content
- Live secrets
- Private keys
- Local override files
- Runtime user config
- Database credentials

## Related Specs
- `DEPLOYMENT_SPEC.md`
- `GITHUB_WORKFLOW_SPEC.md`
- `NGINX_SPEC.md`

## Verification
- Deployment files contain no secrets
- Topology is documented
- Runbooks are up-to-date