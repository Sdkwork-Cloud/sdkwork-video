# SDKWork Video

`sdkwork-video` owns SDKWork video generation core contracts and provider integration boundaries.

## Directory Structure

```
sdkwork-video/
  apis/               # API contracts and specifications
  apps/               # Application roots (PC, Flutter Mobile, H5)
  crates/             # Rust crates (core, service, provider)
  sdks/               # SDK workspaces and generator inputs
  jobs/               # Job definitions and schedules
  tools/              # Developer and operator tools
  plugins/            # Application/runtime plugins
  examples/           # Runnable examples and samples
  configs/            # Configuration templates
  deployments/        # Deployment descriptors
  scripts/            # Build and release scripts
  docs/               # Documentation and ADRs
  tests/              # Cross-package tests
```

## Application Roots

- `apps/sdkwork-video-pc/` - PC browser/desktop/tablet application (React + Tauri)
- `apps/sdkwork-video-flutter-mobile/` - Flutter mobile application (iOS/Android)
- `apps/sdkwork-video-h5/` - H5/Capacitor mobile web application

## Rust Crates

- `crates/sdkwork-video-core-rust/` - Video generation dispatch planning, provider task/result normalization, Drive import planning
- `crates/sdkwork-video-provider-claw-router-rust/` - Claw Router provider gateway for Kling, Vidu, Volcengine, OpenAI
- `crates/sdkwork-video-service-rust/` - Service-level orchestration contracts

Generated videos are planned for Drive `ai_generated` storage and represented as Drive-backed video media resources. Provider URLs are treated as temporary import sources and do not become persisted business media identity.

## Known Issues

- `sdkwork-drive-product` dependency path in `Cargo.toml` references a crate that no longer exists in the `sdkwork-drive` workspace. The `sdkwork-video-core-rust` crate imports `DriveSpaceType`, `PrepareUploaderUploadCommand`, `UploaderActor`, `UploaderRetention`, and `UploaderTarget` from this missing crate. This needs to be resolved by either:
  1. Finding the new location of these types in `sdkwork-drive`
  2. Creating a local adapter crate
  3. Updating the import paths

## Verification

- `cargo test --offline`

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Application Roots

- [apps directory index](apps/README.md)
