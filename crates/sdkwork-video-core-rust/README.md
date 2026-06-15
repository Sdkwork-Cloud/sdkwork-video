# sdkwork_video_core

Domain: content
Capability: video-core
Package type: rust-crate
Status: standard

This crate owns SDKWork video generation dispatch contracts, provider result normalization, Drive-backed generated video import plans, and Drive Uploader command construction.

## Public API

- `.`

## Provider Boundary

This crate does not call provider HTTP APIs. It produces provider dispatch plans that an approved adapter, such as `sdkwork_video_provider_claw_router`, executes through generated SDK clients.

## Media And Drive Contract

Generated videos are planned for Drive `ai_generated` storage. Provider URLs are kept as import sources only; persisted media resources use `source: "drive"` and `kind: "video"`.

## Verification

- `cargo test -p sdkwork_video_core --offline`

