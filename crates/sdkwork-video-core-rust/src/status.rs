#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoGenerationRuntimeStatus {
    Queued,
    Dispatching,
    Submitted,
    Rendering,
    Importing,
    Succeeded,
    Failed,
    CancelRequested,
    Cancelled,
    Expired,
}

impl VideoGenerationRuntimeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Dispatching => "dispatching",
            Self::Submitted => "submitted",
            Self::Rendering => "rendering",
            Self::Importing => "importing",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::CancelRequested => "cancel_requested",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
        }
    }

    pub fn as_drive_sync_status(&self) -> &'static str {
        match self {
            Self::Importing => "importing",
            Self::Succeeded => "imported",
            Self::Failed | Self::Cancelled | Self::Expired => "failed",
            Self::Queued
            | Self::Dispatching
            | Self::Submitted
            | Self::Rendering
            | Self::CancelRequested => "pending",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProviderTaskMode {
    Task,
    Webhook,
}

impl VideoProviderTaskMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Webhook => "webhook",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProviderOperation {
    ViduTextToVideo,
    ViduImageToVideo,
    ViduStartEndToVideo,
    ViduReferenceToVideo,
    KlingVideoGeneration,
    VolcengineContentGeneration,
    OpenAiVideoGeneration,
    ProviderNativeVideoGeneration,
}

impl VideoProviderOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ViduTextToVideo => "vidu.videos.text_to_video",
            Self::ViduImageToVideo => "vidu.videos.image_to_video",
            Self::ViduStartEndToVideo => "vidu.videos.start_end_to_video",
            Self::ViduReferenceToVideo => "vidu.videos.reference_to_video",
            Self::KlingVideoGeneration => "kling.videos.generate",
            Self::VolcengineContentGeneration => "volcengine.contents.generate",
            Self::OpenAiVideoGeneration => "openai.videos.generate",
            Self::ProviderNativeVideoGeneration => "provider_native.videos.generate",
        }
    }
}
