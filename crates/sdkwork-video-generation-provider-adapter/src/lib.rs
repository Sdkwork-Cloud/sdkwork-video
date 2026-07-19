//! L4 generated-SDK adapter for video generation.

mod adapter;
mod normalization;
mod requests;
mod routing;

pub use adapter::VideoGenerationProviderAdapter;
pub use requests::{
    build_kling_video_generation_request, build_openai_video_create_request,
    build_vidu_image_to_video_request, build_vidu_reference_to_video_request,
    build_vidu_start_end_to_video_request, build_vidu_text_to_video_request,
    build_volcengine_video_generation_request,
};
pub use routing::{
    adapter_supports_create_operation, adapter_supports_retrieve_operation,
    resolve_sdk_operation_route, SdkOperationRoute, VIDEO_GENERATION_PROVIDER_ADAPTER_ID,
};
