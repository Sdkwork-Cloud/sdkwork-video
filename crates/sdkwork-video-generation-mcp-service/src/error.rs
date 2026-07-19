use schemars::JsonSchema;
use sdkwork_video_generation_service::VideoGenerationProviderError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpToolError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl McpToolError {
    pub(crate) fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: "invalid_request".into(),
            message: message.into(),
            retryable: false,
        }
    }

    pub(crate) fn task_not_found(handle: &str) -> Self {
        Self {
            code: "task_not_found".into(),
            message: format!("video generation task handle was not found: {handle}"),
            retryable: false,
        }
    }

    pub(crate) fn store_unavailable() -> Self {
        Self {
            code: "task_store_unavailable".into(),
            message: "video MCP task store is unavailable".into(),
            retryable: true,
        }
    }
}

impl From<VideoGenerationProviderError> for McpToolError {
    fn from(error: VideoGenerationProviderError) -> Self {
        let code = match &error {
            VideoGenerationProviderError::InvalidRequest(_) => "invalid_request",
            VideoGenerationProviderError::UnsupportedVendor(_) => "unsupported_vendor",
            VideoGenerationProviderError::UnsupportedModel(_) => "unsupported_model",
            VideoGenerationProviderError::UnsupportedCapability(_) => "unsupported_capability",
            VideoGenerationProviderError::UnsupportedParameter(_) => "unsupported_parameter",
            VideoGenerationProviderError::ProviderNotConfigured(_) => "provider_not_configured",
            VideoGenerationProviderError::ProviderUnavailable(_) => "provider_unavailable",
            VideoGenerationProviderError::RateLimited(_) => "rate_limited",
            VideoGenerationProviderError::Rejected(_) => "rejected",
            VideoGenerationProviderError::Timeout(_) => "timeout",
            VideoGenerationProviderError::Transport(_) => "transport",
            VideoGenerationProviderError::InvalidProviderResponse(_) => "invalid_provider_response",
            VideoGenerationProviderError::Configuration(_) => "configuration",
        };
        Self {
            code: code.into(),
            message: error.to_string(),
            retryable: error.retryable(),
        }
    }
}
