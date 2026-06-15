use sdkwork_video_core::{VideoProviderDispatchPlan, VideoProviderOperation};

pub fn provider_gateway_supports_create_operation(plan: &VideoProviderDispatchPlan) -> bool {
    matches!(
        plan.provider_operation,
        VideoProviderOperation::ViduTextToVideo
            | VideoProviderOperation::ViduImageToVideo
            | VideoProviderOperation::ViduStartEndToVideo
            | VideoProviderOperation::ViduReferenceToVideo
            | VideoProviderOperation::KlingVideoGeneration
            | VideoProviderOperation::VolcengineContentGeneration
            | VideoProviderOperation::OpenAiVideoGeneration
    )
}

pub fn provider_gateway_supports_retrieve_operation(plan: &VideoProviderDispatchPlan) -> bool {
    provider_gateway_supports_create_operation(plan)
}
