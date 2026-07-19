#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum VideoGenerationProviderError {
    #[error("video generation request is invalid: {0}")]
    InvalidRequest(String),
    #[error("video generation vendor is unsupported: {0}")]
    UnsupportedVendor(String),
    #[error("video generation model is unsupported: {0}")]
    UnsupportedModel(String),
    #[error("video generation capability is unsupported: {0}")]
    UnsupportedCapability(String),
    #[error("video generation parameter is unsupported: {0}")]
    UnsupportedParameter(String),
    #[error("video generation provider is not configured: {0}")]
    ProviderNotConfigured(String),
    #[error("video generation provider is unavailable: {0}")]
    ProviderUnavailable(String),
    #[error("video generation provider rate limited the request: {0}")]
    RateLimited(String),
    #[error("video generation provider rejected the request: {0}")]
    Rejected(String),
    #[error("video generation provider timed out: {0}")]
    Timeout(String),
    #[error("video generation provider transport failed: {0}")]
    Transport(String),
    #[error("video generation provider returned an invalid response: {0}")]
    InvalidProviderResponse(String),
    #[error("video generation provider configuration is invalid: {0}")]
    Configuration(String),
}

impl VideoGenerationProviderError {
    pub fn retryable(&self) -> bool {
        matches!(
            self,
            Self::ProviderUnavailable(_)
                | Self::RateLimited(_)
                | Self::Timeout(_)
                | Self::Transport(_)
        )
    }
}

pub type VideoGenerationProviderResult<T> = Result<T, VideoGenerationProviderError>;
