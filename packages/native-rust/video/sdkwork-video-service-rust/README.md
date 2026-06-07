# sdkwork_video_service

Domain: content
Capability: video-service
Package type: rust-crate
Status: standard

This crate owns SDKWork video generation service orchestration contracts. It does not call provider HTTP, generated SDK clients, or persistence directly; it plans runtime steps and persistence bindings around `sdkwork_video_core` contracts.

## Public API

- `.`

## Runtime Contract

The service layer plans:

- generation record creation
- provider dispatch through generated SDK adapter metadata
- provider polling and webhook refresh
- Drive import plans for generated videos
- Drive upload preparation steps
- Drive import completion and generation succeeded steps
- repository method contracts
- notification outbox events

## Verification

- `cargo test -p sdkwork_video_service --offline`
