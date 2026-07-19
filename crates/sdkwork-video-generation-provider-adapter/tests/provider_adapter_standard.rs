use sdkwork_video_generation_provider_adapter::{
    build_kling_video_generation_request, build_vidu_text_to_video_request,
    resolve_sdk_operation_route, VIDEO_GENERATION_PROVIDER_ADAPTER_ID,
};
use sdkwork_video_generation_provider_spi::{
    plan_unified_video_generation_provider_dispatch, VideoGenerationCommand,
    VideoGenerationModelSelection, VideoGenerationVendorParameters, VideoProviderOperation,
    VideoVendorId,
};

fn command(vendor: &str, operation: Option<&str>) -> VideoGenerationCommand {
    VideoGenerationCommand {
        vendor: VideoVendorId::new(vendor).expect("vendor"),
        operation: operation.map(str::to_string),
        model: VideoGenerationModelSelection::named("video-model").expect("model"),
        prompt: "Product reveal".to_string(),
        negative_prompt: None,
        scene: "product_reveal".to_string(),
        resolution: Some("1920x1080".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(8),
        start_image: None,
        end_image: None,
        reference_images: Vec::new(),
        motion_strength: None,
        callback_url: None,
        idempotency_key: None,
        vendor_parameters: None,
    }
}

#[test]
fn sdk_routes_are_owned_only_by_the_adapter() {
    let route =
        resolve_sdk_operation_route(VideoProviderOperation::ViduTextToVideo).expect("vidu route");
    assert_eq!(route.create_resource, "videos_vidu");
    assert_eq!(route.create_method, "create_ent_v2_text2video");
    assert_eq!(
        VIDEO_GENERATION_PROVIDER_ADAPTER_ID,
        "sdkwork-video-generation-provider-adapter"
    );
}

#[test]
fn maps_versioned_vendor_parameters_to_sdk_requests() {
    let mut vidu = command("vidu", Some("text_to_video"));
    vidu.vendor_parameters = Some(VideoGenerationVendorParameters {
        schema: "vidu.video-generation.v1".to_string(),
        values: serde_json::json!({ "seed": 42, "payload": "opaque" }),
    });
    let plan = plan_unified_video_generation_provider_dispatch(&vidu).expect("plan");
    let request = build_vidu_text_to_video_request(&plan).expect("request");
    assert_eq!(request.seed, Some(42));
    assert_eq!(request.payload.as_deref(), Some("opaque"));

    let mut kling = command("kling", None);
    kling.vendor_parameters = Some(VideoGenerationVendorParameters {
        schema: "kling.video-generation.v1".to_string(),
        values: serde_json::json!({ "cfg_scale": 0.7 }),
    });
    let plan = plan_unified_video_generation_provider_dispatch(&kling).expect("plan");
    let request = build_kling_video_generation_request(&plan).expect("request");
    assert_eq!(request.cfg_scale, Some(0.7));
}

#[test]
fn rejects_vendor_parameter_schema_mismatch() {
    let mut vidu = command("vidu", Some("text_to_video"));
    vidu.vendor_parameters = Some(VideoGenerationVendorParameters {
        schema: "kling.video-generation.v1".to_string(),
        values: serde_json::json!({ "cfg_scale": 0.7 }),
    });
    let plan = plan_unified_video_generation_provider_dispatch(&vidu).expect("plan");
    let error = build_vidu_text_to_video_request(&plan).expect_err("schema mismatch");
    assert!(error.to_string().contains("schema"));
}
