use std::sync::Arc;

use sdkwork_video_generation_provider_spi::{
    NormalizedProviderVideoGenerationResult, VideoGenerationCommand, VideoGenerationProvider,
    VideoGenerationProviderDescriptor, VideoGenerationProviderRegistry,
    VideoGenerationProviderResult, VideoProviderDispatchPlan, VideoProviderSubmission,
    VideoVendorId,
};

struct FakeProvider {
    descriptor: VideoGenerationProviderDescriptor,
}

#[async_trait::async_trait]
impl VideoGenerationProvider for FakeProvider {
    fn descriptor(&self) -> &VideoGenerationProviderDescriptor {
        &self.descriptor
    }

    fn validate(&self, _command: &VideoGenerationCommand) -> VideoGenerationProviderResult<()> {
        Ok(())
    }

    async fn generate(
        &self,
        _command: &VideoGenerationCommand,
    ) -> VideoGenerationProviderResult<VideoProviderSubmission> {
        unreachable!("registry test does not dispatch")
    }

    async fn retrieve(
        &self,
        _dispatch_plan: &VideoProviderDispatchPlan,
        _provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult> {
        unreachable!("registry test does not retrieve")
    }
}

#[test]
fn registry_selects_an_injected_provider_by_vendor() {
    let provider = Arc::new(FakeProvider {
        descriptor: VideoGenerationProviderDescriptor {
            id: "fake-video-provider".to_string(),
            vendors: vec![VideoVendorId::new("vidu").expect("vendor")],
            capabilities: Vec::new(),
        },
    });
    let registry = VideoGenerationProviderRegistry::builder()
        .register(provider)
        .expect("provider")
        .default_provider("fake-video-provider")
        .build()
        .expect("registry");
    let selected = registry
        .select_for_vendor(&VideoVendorId::new("vidu").expect("vendor"))
        .expect("selected provider");
    assert_eq!(selected.descriptor().id, "fake-video-provider");
}
