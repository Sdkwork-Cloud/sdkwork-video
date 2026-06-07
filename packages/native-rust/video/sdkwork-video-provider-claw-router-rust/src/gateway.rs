use clawrouter_open_sdk::{SdkworkAiClient, SdkworkError};
use sdkwork_video_core::{
    NormalizedProviderVideoGenerationResult, VideoProviderDispatchPlan, VideoProviderOperation,
};

use crate::{
    normalize::{
        normalize_kling_generation_task, normalize_openai_video,
        normalize_vidu_task_creations_response, normalize_vidu_video_generation_task,
        normalize_volcengine_create_response, normalize_volcengine_generation_task,
        sdk_error_from_normalization,
    },
    requests::{
        build_kling_video_generation_request, build_openai_video_create_request,
        build_vidu_image_to_video_request, build_vidu_reference_to_video_request,
        build_vidu_start_end_to_video_request, build_vidu_text_to_video_request,
        build_volcengine_video_generation_request,
    },
};

#[derive(Clone)]
pub struct ClawRouterVideoProviderGateway {
    client: SdkworkAiClient,
}

impl ClawRouterVideoProviderGateway {
    pub fn new(client: SdkworkAiClient) -> Self {
        Self { client }
    }

    pub async fn create_video_generation(
        &self,
        plan: &VideoProviderDispatchPlan,
    ) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
        match plan.provider_operation {
            VideoProviderOperation::ViduTextToVideo => {
                let request = build_vidu_text_to_video_request(plan);
                let task = self
                    .client
                    .videos_vidu()
                    .create_ent_v2_text2video(&request)
                    .await?;
                normalize_vidu_video_generation_task(plan, task)
            }
            VideoProviderOperation::ViduImageToVideo => {
                let request = build_vidu_image_to_video_request(plan);
                let task = self
                    .client
                    .videos_vidu()
                    .create_ent_v2_img2video(&request)
                    .await?;
                normalize_vidu_video_generation_task(plan, task)
            }
            VideoProviderOperation::ViduStartEndToVideo => {
                let request = build_vidu_start_end_to_video_request(plan);
                let task = self
                    .client
                    .videos_vidu()
                    .create_ent_v2_start_end2video(&request)
                    .await?;
                normalize_vidu_video_generation_task(plan, task)
            }
            VideoProviderOperation::ViduReferenceToVideo => {
                let request = build_vidu_reference_to_video_request(plan);
                let task = self
                    .client
                    .videos_vidu()
                    .create_ent_v2_reference2video(&request)
                    .await?;
                normalize_vidu_video_generation_task(plan, task)
            }
            VideoProviderOperation::KlingVideoGeneration => {
                let request = build_kling_video_generation_request(plan);
                let task = self
                    .client
                    .videos_kling()
                    .create_v1_videos_generation(&request)
                    .await?;
                normalize_kling_generation_task(plan, task)
            }
            VideoProviderOperation::VolcengineContentGeneration => {
                let request = build_volcengine_video_generation_request(plan);
                let task = self
                    .client
                    .videos_volcengine()
                    .create_api_v3_contents_generations_task(&request)
                    .await?;
                normalize_volcengine_create_response(plan, task)
            }
            VideoProviderOperation::OpenAiVideoGeneration => {
                let request = build_openai_video_create_request(plan);
                let task = self.client.video().create(&request).await?;
                normalize_openai_video(plan, task)
            }
            VideoProviderOperation::ProviderNativeVideoGeneration => Err(
                sdk_error_from_normalization(
                    "video provider operation is not exposed by the generated Claw Router SDK gateway",
                ),
            ),
        }
    }

    pub async fn retrieve_video_generation(
        &self,
        plan: &VideoProviderDispatchPlan,
        provider_task_id: &str,
    ) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
        match plan.provider_operation {
            VideoProviderOperation::ViduTextToVideo
            | VideoProviderOperation::ViduImageToVideo
            | VideoProviderOperation::ViduStartEndToVideo
            | VideoProviderOperation::ViduReferenceToVideo => {
                let task = self
                    .client
                    .videos_vidu()
                    .list_ent_v2_tasks_creations(provider_task_id)
                    .await?;
                normalize_vidu_task_creations_response(plan, task)
            }
            VideoProviderOperation::KlingVideoGeneration => {
                let task = self
                    .client
                    .videos_kling()
                    .list_v1_videos_generations(provider_task_id)
                    .await?;
                normalize_kling_generation_task(plan, task)
            }
            VideoProviderOperation::VolcengineContentGeneration => {
                let task = self
                    .client
                    .videos_volcengine()
                    .list_api_v3_contents_generations_tasks(provider_task_id)
                    .await?;
                normalize_volcengine_generation_task(plan, task)
            }
            VideoProviderOperation::OpenAiVideoGeneration => {
                let task = self.client.video().retrieve(provider_task_id).await?;
                normalize_openai_video(plan, task)
            }
            VideoProviderOperation::ProviderNativeVideoGeneration => Err(
                sdk_error_from_normalization(
                    "video provider operation does not support task retrieval through the generated Claw Router SDK gateway",
                ),
            ),
        }
    }
}
