use rmcp::model::{
    GetPromptResult, ListPromptsResult, ListResourcesResult, Prompt, PromptMessage, Resource,
    ResourceContents, Role,
};
use sdkwork_video_generation_service::{
    VideoGenerationProviderCapability, VideoGenerationProviderDescriptor,
};

pub(crate) const CAPABILITIES_URI: &str = "sdkwork://video/generation/capabilities";
pub(crate) const VENDORS_URI: &str = "sdkwork://video/generation/vendors";
pub(crate) const GENERATION_PROMPT: &str = "video.generation.request";

pub(crate) fn resources() -> ListResourcesResult {
    ListResourcesResult::with_all_items(vec![
        Resource::new(CAPABILITIES_URI, "video-generation-capabilities")
            .with_title("Video generation capabilities")
            .with_mime_type("application/json"),
        Resource::new(VENDORS_URI, "video-generation-vendors")
            .with_title("Video generation vendors")
            .with_mime_type("application/json"),
    ])
}

pub(crate) fn catalog(descriptors: Vec<VideoGenerationProviderDescriptor>) -> serde_json::Value {
    let providers = descriptors.into_iter().map(|descriptor| serde_json::json!({
        "vendors": descriptor.vendors.into_iter().map(|vendor| vendor.to_string()).collect::<Vec<_>>(),
        "capabilities": descriptor.capabilities.into_iter().map(capability_name).collect::<Vec<_>>(),
    })).collect::<Vec<_>>();
    serde_json::json!({
        "domain": "video",
        "tools": ["video.generate", "video.retrieve", "video.cancel", "video.capabilities"],
        "transports": ["stdio", "streamable-http-sse"],
        "providers": providers
    })
}

pub(crate) fn read(
    uri: &str,
    descriptors: Vec<VideoGenerationProviderDescriptor>,
) -> Option<ResourceContents> {
    let catalog = catalog(descriptors);
    let value = match uri {
        CAPABILITIES_URI => catalog,
        VENDORS_URI => catalog.get("providers")?.clone(),
        _ => return None,
    };
    Some(
        ResourceContents::text(serde_json::to_string_pretty(&value).ok()?, uri)
            .with_mime_type("application/json"),
    )
}

pub(crate) fn prompts() -> ListPromptsResult {
    ListPromptsResult::with_all_items(vec![Prompt::new(
        GENERATION_PROMPT,
        Some("Prepare a provider-neutral video generation request for video.generate."),
        None,
    )])
}

pub(crate) fn prompt() -> GetPromptResult {
    GetPromptResult::new(vec![PromptMessage::new_text(Role::User, "Create a video generation request. Inspect sdkwork://video/generation/vendors, select an operation supported by the chosen vendor, keep provider-only fields inside vendorParameters with its schema identifier, and invoke video.generate.")]).with_description("Provider-neutral video generation request workflow")
}

fn capability_name(capability: VideoGenerationProviderCapability) -> &'static str {
    match capability {
        VideoGenerationProviderCapability::TextToVideo => "text-to-video",
        VideoGenerationProviderCapability::ImageToVideo => "image-to-video",
        VideoGenerationProviderCapability::ReferenceToVideo => "reference-to-video",
        VideoGenerationProviderCapability::StartEndToVideo => "start-end-to-video",
        VideoGenerationProviderCapability::Polling => "polling",
        VideoGenerationProviderCapability::Webhook => "webhook",
        VideoGenerationProviderCapability::Cancellation => "cancellation",
    }
}
