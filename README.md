# SDKWork Video

`sdkwork-video` owns SDKWork video generation core contracts and provider integration boundaries.

The current Rust implementation is intentionally focused on the provider-generation layer:

- `packages/native-rust/video/sdkwork-video-core-rust` owns video generation dispatch planning, provider task/result normalization, generated video Drive import planning, and Drive Uploader command construction.
- `packages/native-rust/video/sdkwork-video-provider-claw-router-rust` owns the Claw Router provider gateway and calls `clawrouter_open_sdk` generated Rust SDK APIs for Kling, Vidu, Volcengine, and OpenAI-compatible video generation. Product code must use this gateway or another approved generated-SDK adapter instead of raw provider HTTP.
- `packages/native-rust/video/sdkwork-video-service-rust` owns service-level orchestration contracts for create, polling refresh, webhook refresh, Drive import planning, Drive upload preparation, Drive import completion, persistence method planning, and notification outbox planning.

Generated videos are planned for Drive `ai_generated` storage and represented as Drive-backed video media resources. Provider URLs are treated as temporary import sources and do not become persisted business media identity.

## Verification

- `cargo test --offline`
