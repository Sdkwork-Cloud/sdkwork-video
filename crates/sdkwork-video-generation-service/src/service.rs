use std::sync::Arc;

use async_trait::async_trait;
use sdkwork_video_generation_provider_spi::{
    NormalizedProviderVideoGenerationResult, VideoGenerationCommand,
    VideoGenerationProviderDescriptor, VideoGenerationProviderError,
    VideoGenerationProviderRegistry, VideoGenerationProviderResult, VideoProviderDispatchPlan,
    VideoProviderSubmission, VideoVendorId,
};

#[async_trait]
pub trait VideoGenerationServicePort: Send + Sync {
    async fn generate(
        &self,
        command: VideoGenerationCommand,
    ) -> VideoGenerationProviderResult<VideoProviderSubmission>;

    async fn retrieve(
        &self,
        dispatch_plan: &VideoProviderDispatchPlan,
        provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult>;

    async fn cancel(
        &self,
        dispatch_plan: &VideoProviderDispatchPlan,
        provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult>;

    fn provider_descriptors(&self) -> Vec<VideoGenerationProviderDescriptor>;
}

#[derive(Clone)]
pub struct VideoGenerationService {
    providers: Arc<VideoGenerationProviderRegistry>,
}

impl VideoGenerationService {
    pub fn new(providers: VideoGenerationProviderRegistry) -> Self {
        Self {
            providers: Arc::new(providers),
        }
    }

    fn provider_for_dispatch(
        &self,
        dispatch_plan: &VideoProviderDispatchPlan,
    ) -> VideoGenerationProviderResult<
        Arc<dyn sdkwork_video_generation_provider_spi::VideoGenerationProvider>,
    > {
        if !dispatch_plan.provider_id.trim().is_empty() {
            return self.providers.select_by_id(&dispatch_plan.provider_id);
        }
        let vendor = VideoVendorId::new(&dispatch_plan.provider_code)
            .map_err(|message| VideoGenerationProviderError::InvalidRequest(message.to_string()))?;
        self.providers.select_for_vendor(&vendor)
    }
}

#[async_trait]
impl VideoGenerationServicePort for VideoGenerationService {
    async fn generate(
        &self,
        command: VideoGenerationCommand,
    ) -> VideoGenerationProviderResult<VideoProviderSubmission> {
        let provider = self.providers.select_for_vendor(&command.vendor)?;
        provider.validate(&command)?;
        provider.generate(&command).await
    }

    async fn retrieve(
        &self,
        dispatch_plan: &VideoProviderDispatchPlan,
        provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult> {
        if provider_task_id.trim().is_empty() {
            return Err(VideoGenerationProviderError::InvalidRequest(
                "provider_task_id is required".to_string(),
            ));
        }
        self.provider_for_dispatch(dispatch_plan)?
            .retrieve(dispatch_plan, provider_task_id.trim())
            .await
    }

    async fn cancel(
        &self,
        dispatch_plan: &VideoProviderDispatchPlan,
        provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult> {
        if provider_task_id.trim().is_empty() {
            return Err(VideoGenerationProviderError::InvalidRequest(
                "provider_task_id is required".to_string(),
            ));
        }
        self.provider_for_dispatch(dispatch_plan)?
            .cancel(dispatch_plan, provider_task_id.trim())
            .await
    }

    fn provider_descriptors(&self) -> Vec<VideoGenerationProviderDescriptor> {
        self.providers.descriptors()
    }
}
