use std::sync::Arc;

use sdkwork_video_generation_service::{
    plan_unified_video_generation_provider_dispatch, NormalizedProviderVideoGenerationResult,
    VideoGenerationCommand, VideoGenerationModelSelection, VideoGenerationProvider,
    VideoGenerationProviderDescriptor, VideoGenerationProviderRegistry,
    VideoGenerationProviderResult, VideoGenerationRuntimeStatus, VideoGenerationService,
    VideoGenerationServicePort, VideoProviderDispatchPlan, VideoProviderSubmission, VideoVendorId,
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
        command: &VideoGenerationCommand,
    ) -> VideoGenerationProviderResult<VideoProviderSubmission> {
        let mut dispatch_plan =
            plan_unified_video_generation_provider_dispatch(command).expect("dispatch plan");
        dispatch_plan.provider_id = self.descriptor.id.clone();
        Ok(VideoProviderSubmission {
            dispatch_plan,
            result: submitted("vidu", "video-task-1"),
        })
    }

    async fn retrieve(
        &self,
        dispatch_plan: &VideoProviderDispatchPlan,
        provider_task_id: &str,
    ) -> VideoGenerationProviderResult<NormalizedProviderVideoGenerationResult> {
        Ok(submitted(&dispatch_plan.provider_code, provider_task_id))
    }
}

#[tokio::test]
async fn unified_service_routes_generate_and_retrieve_through_injected_spi() {
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
    let service = VideoGenerationService::new(registry);
    let command = VideoGenerationCommand {
        vendor: VideoVendorId::new("vidu").expect("vendor"),
        operation: Some("text_to_video".to_string()),
        model: VideoGenerationModelSelection::named("vidu2.0").expect("model"),
        prompt: "Product reveal".to_string(),
        negative_prompt: None,
        scene: "product_reveal".to_string(),
        resolution: Some("1080p".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(8),
        start_image: None,
        end_image: None,
        reference_images: Vec::new(),
        motion_strength: None,
        callback_url: None,
        idempotency_key: None,
        vendor_parameters: None,
    };
    let submission = service.generate(command).await.expect("submission");
    assert_eq!(submission.dispatch_plan.provider_id, "fake-video-provider");
    let result = service
        .retrieve(&submission.dispatch_plan, "video-task-1")
        .await
        .expect("retrieved");
    assert_eq!(result.provider_task_id.as_deref(), Some("video-task-1"));
}

fn submitted(vendor: &str, task_id: &str) -> NormalizedProviderVideoGenerationResult {
    NormalizedProviderVideoGenerationResult {
        provider_code: vendor.to_string(),
        provider_task_id: Some(task_id.to_string()),
        provider_status: Some("submitted".to_string()),
        provider_state: None,
        status: VideoGenerationRuntimeStatus::Submitted,
        provider_terminal: false,
        ready_for_drive_import: false,
        outputs: Vec::new(),
        error_code: None,
        error_message: None,
    }
}
