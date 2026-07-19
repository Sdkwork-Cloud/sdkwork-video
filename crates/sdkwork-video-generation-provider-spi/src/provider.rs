use async_trait::async_trait;

use crate::{
    NormalizedProviderVideoGenerationResult, VideoGenerationCommand, VideoGenerationProviderError,
    VideoGenerationProviderResult, VideoProviderDispatchPlan, VideoVendorId,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VideoGenerationProviderCapability {
    TextToVideo,
    ImageToVideo,
    ReferenceToVideo,
    StartEndToVideo,
    Polling,
    Webhook,
    Cancellation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationProviderDescriptor {
    pub id: String,
    pub vendors: Vec<VideoVendorId>,
    pub capabilities: Vec<VideoGenerationProviderCapability>,
}

impl VideoGenerationProviderDescriptor {
    pub fn supports_vendor(&self, vendor: &VideoVendorId) -> bool {
        self.vendors.iter().any(|candidate| candidate == vendor)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationProviderHealth {
    pub available: bool,
    pub detail: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoProviderSubmission {
    pub dispatch_plan: VideoProviderDispatchPlan,
    pub result: NormalizedProviderVideoGenerationResult,
}

#[async_trait]
pub trait VideoGenerationProvider: Send + Sync {
    fn descriptor(&self) -> &VideoGenerationProviderDescriptor;

    fn validate(&self, command: &VideoGenerationCommand) -> VideoGenerationProviderResult<()>;

    async fn generate(
        &self,
        command: &VideoGenerationCommand,
    ) -> VideoGenerationProviderResult<VideoProviderSubmission>;

    async fn retrieve(
        &self,
        dispatch_plan: &VideoProviderDispatchPlan,
        provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult>;

    async fn cancel(
        &self,
        _dispatch_plan: &VideoProviderDispatchPlan,
        _provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult> {
        Err(VideoGenerationProviderError::UnsupportedCapability(
            "cancellation".to_string(),
        ))
    }

    async fn health(&self) -> VideoGenerationProviderResult<VideoGenerationProviderHealth> {
        Ok(VideoGenerationProviderHealth {
            available: true,
            detail: None,
        })
    }
}
