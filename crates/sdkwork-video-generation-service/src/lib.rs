//! Unified application service for video generation providers.

mod service;

pub use sdkwork_video_generation_provider_spi::*;
pub use service::{VideoGenerationService, VideoGenerationServicePort};
