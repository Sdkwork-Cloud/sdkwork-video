# SDKWork Video H5 Application

## Purpose
Phone-first H5 and Capacitor iOS/Android application root for SDKWork Video.

## Architecture
This application root follows `APP_H5_ARCHITECTURE_SPEC.md` for H5 browser, WeChat-H5, embedded WebView, and Capacitor iOS/Android targets.

## Directory Structure
```
apps/sdkwork-video-h5/
  .sdkwork/           # Workspace metadata and skills
  bin/                # Cross-platform operational scripts
  config/             # Configuration templates by runtime target
  docs/               # Architecture notes and runbooks
  packages/           # Reusable runtime, shell, app, console, admin packages
  public/             # Browser-served static assets
  scripts/            # Build, validation, generation utilities
  sdks/               # SDK workspaces and generator inputs
  specs/              # Local component/application specs
  src/                # Root shell entry and composition boundary
  tests/              # Application-level integration tests
```

## Package Taxonomy
- `sdkwork-video-h5-core/` - Core runtime, SDK factories, TokenManager binding
- `sdkwork-video-h5-commons/` - Shared mobile UI/runtime primitives
- `sdkwork-video-h5-shell/` - App shell, route assembly, navigation
- `sdkwork-video-h5-<capability>/` - App/user domain features
- `sdkwork-video-h5-console-<capability>/` - User-facing management console
- `sdkwork-video-h5-admin-<capability>/` - Internal admin operations
- `sdkwork-video-h5-capacitor/` - Capacitor native host

## Related Specs
- `APP_H5_ARCHITECTURE_SPEC.md`
- `APP_MOBILE_REACT_UI_SPEC.md`
- `APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`
- `APP_SDK_INTEGRATION_SPEC.md`
- `CONFIG_SPEC.md`

## Commands
```bash
pnpm install
pnpm dev
pnpm build
```
