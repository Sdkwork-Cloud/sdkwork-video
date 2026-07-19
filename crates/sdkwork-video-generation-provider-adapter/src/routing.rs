use sdkwork_video_generation_provider_spi::{VideoProviderDispatchPlan, VideoProviderOperation};

pub const VIDEO_GENERATION_PROVIDER_ADAPTER_ID: &str = "sdkwork-video-generation-provider-adapter";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SdkOperationRoute {
    pub create_resource: &'static str,
    pub create_method: &'static str,
    pub retrieve_resource: &'static str,
    pub retrieve_method: &'static str,
}

pub fn resolve_sdk_operation_route(operation: VideoProviderOperation) -> Option<SdkOperationRoute> {
    match operation {
        VideoProviderOperation::ViduTextToVideo => Some(route(
            "videos_vidu",
            "create_ent_v2_text2video",
            "videos_vidu",
            "list_ent_v2_tasks_creations",
        )),
        VideoProviderOperation::ViduImageToVideo => Some(route(
            "videos_vidu",
            "create_ent_v2_img2video",
            "videos_vidu",
            "list_ent_v2_tasks_creations",
        )),
        VideoProviderOperation::ViduStartEndToVideo => Some(route(
            "videos_vidu",
            "create_ent_v2_start_end2video",
            "videos_vidu",
            "list_ent_v2_tasks_creations",
        )),
        VideoProviderOperation::ViduReferenceToVideo => Some(route(
            "videos_vidu",
            "create_ent_v2_reference2video",
            "videos_vidu",
            "list_ent_v2_tasks_creations",
        )),
        VideoProviderOperation::KlingVideoGeneration => Some(route(
            "videos_kling",
            "create_v1_videos_generation",
            "videos_kling",
            "list_v1_videos_generations",
        )),
        VideoProviderOperation::VolcengineContentGeneration => Some(route(
            "videos_volcengine",
            "create_api_v3_contents_generations_task",
            "videos_volcengine",
            "list_api_v3_contents_generations_tasks",
        )),
        VideoProviderOperation::OpenAiVideoGeneration => {
            Some(route("video", "create", "video", "retrieve"))
        }
        VideoProviderOperation::ProviderNativeVideoGeneration => None,
    }
}

pub fn adapter_supports_create_operation(plan: &VideoProviderDispatchPlan) -> bool {
    resolve_sdk_operation_route(plan.provider_operation).is_some()
}

pub fn adapter_supports_retrieve_operation(plan: &VideoProviderDispatchPlan) -> bool {
    resolve_sdk_operation_route(plan.provider_operation).is_some()
}

const fn route(
    create_resource: &'static str,
    create_method: &'static str,
    retrieve_resource: &'static str,
    retrieve_method: &'static str,
) -> SdkOperationRoute {
    SdkOperationRoute {
        create_resource,
        create_method,
        retrieve_resource,
        retrieve_method,
    }
}
