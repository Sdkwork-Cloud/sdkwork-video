//! Stable provider port for SDKWork video generation.

mod error;
mod provider;
mod registry;

pub use error::{VideoGenerationProviderError, VideoGenerationProviderResult};
pub use provider::{
    VideoGenerationProvider, VideoGenerationProviderCapability, VideoGenerationProviderDescriptor,
    VideoGenerationProviderHealth, VideoProviderSubmission,
};
pub use registry::{VideoGenerationProviderRegistry, VideoGenerationProviderRegistryBuilder};
pub use sdkwork_video_core::{
    normalize_provider_task_video_generation_result,
    plan_unified_video_generation_provider_dispatch, plan_video_generation_provider_dispatch,
    GeneratedVideoOutput, NormalizedProviderVideoGenerationResult, ProviderGeneratedVideoAsset,
    ProviderTaskErrorSnapshot, ProviderTaskSnapshot, VideoGenerationCommand,
    VideoGenerationCreateCommand, VideoGenerationModelSelection, VideoGenerationRuntimeStatus,
    VideoGenerationVendorParameters, VideoProviderDispatchPlan, VideoProviderOperation,
    VideoProviderTaskMode, VideoVendorId,
};
