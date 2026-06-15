# Crates Directory

## Purpose
Rust crates, including route crates, service crates, repository crates, API server/service-host/native-host/Tauri-host/gateway/worker crates, and reusable Rust libraries.

## Owner
Rust developers and backend teams.

## Allowed Content
- Rust crate source code
- Cargo.toml manifests
- Rust tests and benchmarks
- Rust documentation

## Forbidden Content
- Non-Rust source code
- Generated SDK output
- Frontend application code
- Deployment configuration

## Related Specs
- `RUST_CODE_SPEC.md`
- `RUST_RPC_SPEC.md`
- `WEB_BACKEND_SPEC.md`
- `SDK_SPEC.md`

## Verification
- Crates follow Rust naming conventions
- `src/lib.rs` is a module assembly boundary
- Crates are named by responsibility (e.g., `sdkwork-<domain>-<capability>-service`)