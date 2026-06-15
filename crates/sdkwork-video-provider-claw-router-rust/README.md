# sdkwork_video_provider_claw_router

Domain: content
Capability: video-provider-claw-router
Package type: rust-crate
Status: standard

This crate owns the SDKWork Video provider gateway backed by the generated Claw Router Rust SDK.

## Public API

- `.`

## Required SDK Surface

- Generated Rust SDK crate: `clawrouter-open-sdk`.
- Generated client resources used by this adapter: `video.create`, `video.retrieve`, `videos_kling.create_v1_videos_generation`, `videos_kling.list_v1_videos_generations`, `videos_vidu.create_ent_v2_text2video`, `videos_vidu.create_ent_v2_img2video`, `videos_vidu.create_ent_v2_start_end2video`, `videos_vidu.create_ent_v2_reference2video`, `videos_vidu.list_ent_v2_tasks_creations`, `videos_volcengine.create_api_v3_contents_generations_task`, and `videos_volcengine.list_api_v3_contents_generations_tasks`.
- Product code must use this adapter or another approved generated-SDK adapter instead of raw Claw Router HTTP.

## Verification

- `cargo test -p sdkwork_video_provider_claw_router --offline`
