use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{
    VideoGenerationProvider, VideoGenerationProviderError, VideoGenerationProviderResult,
    VideoVendorId,
};

#[derive(Clone, Default)]
pub struct VideoGenerationProviderRegistry {
    providers: BTreeMap<String, Arc<dyn VideoGenerationProvider>>,
    default_provider_id: Option<String>,
}

impl VideoGenerationProviderRegistry {
    pub fn builder() -> VideoGenerationProviderRegistryBuilder {
        VideoGenerationProviderRegistryBuilder::default()
    }

    pub fn select_for_vendor(
        &self,
        vendor: &VideoVendorId,
    ) -> VideoGenerationProviderResult<Arc<dyn VideoGenerationProvider>> {
        if let Some(provider) = self
            .default_provider_id
            .as_deref()
            .and_then(|id| self.providers.get(id))
            .filter(|provider| provider.descriptor().supports_vendor(vendor))
        {
            return Ok(provider.clone());
        }
        self.providers
            .values()
            .find(|provider| provider.descriptor().supports_vendor(vendor))
            .cloned()
            .ok_or_else(|| VideoGenerationProviderError::UnsupportedVendor(vendor.to_string()))
    }

    pub fn select_by_id(
        &self,
        provider_id: &str,
    ) -> VideoGenerationProviderResult<Arc<dyn VideoGenerationProvider>> {
        self.providers.get(provider_id).cloned().ok_or_else(|| {
            VideoGenerationProviderError::ProviderNotConfigured(provider_id.to_string())
        })
    }

    pub fn descriptors(&self) -> Vec<crate::VideoGenerationProviderDescriptor> {
        self.providers
            .values()
            .map(|provider| provider.descriptor().clone())
            .collect()
    }
}

#[derive(Default)]
pub struct VideoGenerationProviderRegistryBuilder {
    providers: BTreeMap<String, Arc<dyn VideoGenerationProvider>>,
    default_provider_id: Option<String>,
}

impl VideoGenerationProviderRegistryBuilder {
    pub fn register(
        mut self,
        provider: Arc<dyn VideoGenerationProvider>,
    ) -> VideoGenerationProviderResult<Self> {
        let id = provider.descriptor().id.trim().to_string();
        if id.is_empty() {
            return Err(VideoGenerationProviderError::Configuration(
                "provider id is required".to_string(),
            ));
        }
        if self.providers.insert(id.clone(), provider).is_some() {
            return Err(VideoGenerationProviderError::Configuration(format!(
                "duplicate provider id: {id}"
            )));
        }
        Ok(self)
    }

    pub fn default_provider(mut self, provider_id: impl Into<String>) -> Self {
        self.default_provider_id = Some(provider_id.into());
        self
    }

    pub fn build(self) -> VideoGenerationProviderResult<VideoGenerationProviderRegistry> {
        if let Some(default_provider_id) = self.default_provider_id.as_deref() {
            if !self.providers.contains_key(default_provider_id) {
                return Err(VideoGenerationProviderError::Configuration(format!(
                    "default provider is not registered: {default_provider_id}"
                )));
            }
        }
        Ok(VideoGenerationProviderRegistry {
            providers: self.providers,
            default_provider_id: self.default_provider_id,
        })
    }
}
