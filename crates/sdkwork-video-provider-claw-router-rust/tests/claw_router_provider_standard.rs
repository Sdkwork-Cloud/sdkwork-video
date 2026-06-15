use clawrouter_open_sdk::{
    KlingVideoGenerationRequest, OpenAiVideoCreateRequest, SdkworkAiClient, ViduTextToVideoRequest,
    VolcengineContentGenerationTaskCreateRequest,
};
use sdkwork_video_core::{
    plan_video_generation_provider_dispatch, VideoGenerationCreateCommand, VideoProviderOperation,
};
use sdkwork_video_provider_claw_router::{
    build_kling_video_generation_request, build_openai_video_create_request,
    build_vidu_image_to_video_request, build_vidu_reference_to_video_request,
    build_vidu_start_end_to_video_request, build_vidu_text_to_video_request,
    build_volcengine_video_generation_request, provider_gateway_supports_create_operation,
    provider_gateway_supports_retrieve_operation, CLAW_ROUTER_OPEN_SDK_CRATE,
    CLAW_ROUTER_VIDEO_GENERATION_METHODS,
};

#[test]
fn keeps_lib_rs_as_public_module_boundary() {
    let lib_rs = include_str!("../src/lib.rs");

    assert!(
        lib_rs.lines().count() <= 80,
        "sdkwork-video-provider-claw-router-rust src/lib.rs must stay a small module assembly boundary"
    );
    for forbidden in ["pub struct ", "pub fn ", "impl ", "use clawrouter_open_sdk"] {
        assert!(
            !lib_rs.contains(forbidden),
            "src/lib.rs must not contain authored provider logic marker {forbidden}"
        );
    }
    assert!(lib_rs.contains("pub mod "));
    assert!(lib_rs.contains("pub use "));
}

#[test]
fn exposes_generated_claw_router_sdk_as_video_provider_boundary() {
    let client = SdkworkAiClient::new_with_base_url("http://127.0.0.1:18080")
        .expect("generated claw router SDK client should construct");
    let _kling = client.videos_kling();
    let _vidu = client.videos_vidu();
    let _volcengine = client.videos_volcengine();
    let _video = client.video();

    assert_eq!(CLAW_ROUTER_OPEN_SDK_CRATE, "clawrouter_open_sdk");
    assert!(CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"video.create"));
    assert!(CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"video.retrieve"));
    assert!(
        CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"videos_kling.create_v1_videos_generation")
    );
    assert!(CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"videos_vidu.create_ent_v2_text2video"));
    assert!(CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"videos_vidu.create_ent_v2_img2video"));
    assert!(
        CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"videos_vidu.create_ent_v2_start_end2video")
    );
    assert!(
        CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"videos_vidu.create_ent_v2_reference2video")
    );
    assert!(
        CLAW_ROUTER_VIDEO_GENERATION_METHODS.contains(&"videos_vidu.list_ent_v2_tasks_creations")
    );
    assert!(CLAW_ROUTER_VIDEO_GENERATION_METHODS
        .contains(&"videos_volcengine.create_api_v3_contents_generations_task"));
    assert!(CLAW_ROUTER_VIDEO_GENERATION_METHODS
        .contains(&"videos_volcengine.list_api_v3_contents_generations_tasks"));
}

#[test]
fn maps_video_dispatch_plan_to_generated_vidu_text_to_video_request() {
    let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Cinematic product reveal".to_string(),
        negative_prompt: None,
        scene: "product_reveal".to_string(),
        provider_code: Some("vidu".to_string()),
        operation: Some("text_to_video".to_string()),
        model: Some("vidu2.0".to_string()),
        resolution: Some("1080p".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(8),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: Some("normal".to_string()),
        webhook_url: Some("https://app.example.com/hooks/vidu".to_string()),
        idempotency_key: Some("idem-vidu-001".to_string()),
    })
    .expect("dispatch plan should build");

    let request = build_vidu_text_to_video_request(&plan);
    assert_eq!(request.model, "vidu2.0");
    assert_eq!(request.prompt, "Cinematic product reveal");
    assert_eq!(request.resolution.as_deref(), Some("1080p"));
    assert_eq!(request.aspect_ratio.as_deref(), Some("16:9"));
    assert_eq!(request.duration, Some(8));
    assert_eq!(request.movement_amplitude.as_deref(), Some("normal"));
    let payload = request
        .payload
        .as_deref()
        .expect("provider payload should carry trace fields");
    let payload: serde_json::Value =
        serde_json::from_str(payload).expect("provider payload should be json");
    assert_eq!(
        payload
            .get("idempotencyKey")
            .and_then(|value| value.as_str()),
        Some("idem-vidu-001")
    );
    assert_eq!(
        payload.get("scene").and_then(|value| value.as_str()),
        Some("product_reveal")
    );
    assert_eq!(
        request.callback_url.as_deref(),
        Some("https://app.example.com/hooks/vidu")
    );

    let serialized = serde_json::to_value(ViduTextToVideoRequest {
        aspect_ratio: request.aspect_ratio,
        callback_url: request.callback_url,
        duration: request.duration,
        model: request.model,
        movement_amplitude: request.movement_amplitude,
        payload: request.payload,
        prompt: request.prompt,
        resolution: request.resolution,
        seed: request.seed,
    })
    .expect("generated SDK request should serialize");
    assert_eq!(
        serialized.get("prompt").and_then(|value| value.as_str()),
        Some("Cinematic product reveal")
    );
}

#[test]
fn keeps_vidu_scene_payload_when_idempotency_key_is_not_supplied() {
    let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Cinematic product reveal".to_string(),
        negative_prompt: None,
        scene: "product_reveal_without_idempotency".to_string(),
        provider_code: Some("vidu".to_string()),
        operation: Some("text_to_video".to_string()),
        model: Some("vidu2.0".to_string()),
        resolution: Some("1080p".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(8),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: Some("normal".to_string()),
        webhook_url: Some("https://app.example.com/hooks/vidu".to_string()),
        idempotency_key: None,
    })
    .expect("dispatch plan should build without idempotency key");

    let request = build_vidu_text_to_video_request(&plan);
    let payload = request
        .payload
        .as_deref()
        .expect("vidu payload should keep scene trace metadata");
    let payload: serde_json::Value =
        serde_json::from_str(payload).expect("vidu payload should be json");

    assert_eq!(
        payload.get("scene").and_then(|value| value.as_str()),
        Some("product_reveal_without_idempotency")
    );
    assert!(
        payload.get("idempotencyKey").is_none(),
        "vidu payload should not synthesize an idempotency key"
    );
}

#[test]
fn maps_video_dispatch_plan_to_generated_kling_request() {
    let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Neon launch clip".to_string(),
        negative_prompt: Some("blur".to_string()),
        scene: "launch".to_string(),
        provider_code: Some("kling".to_string()),
        operation: None,
        model: Some("kling-v2".to_string()),
        resolution: None,
        aspect_ratio: Some("9:16".to_string()),
        duration_seconds: Some(5),
        start_image: Some("https://assets.example.com/start.png".to_string()),
        end_image: Some("https://assets.example.com/end.png".to_string()),
        reference_images: vec![],
        motion_strength: Some("pro".to_string()),
        webhook_url: Some("https://app.example.com/hooks/kling".to_string()),
        idempotency_key: None,
    })
    .expect("dispatch plan should build");

    let request = build_kling_video_generation_request(&plan);
    assert_eq!(request.model.as_deref(), Some("kling-v2"));
    assert_eq!(request.prompt, "Neon launch clip");
    assert_eq!(request.negative_prompt.as_deref(), Some("blur"));
    assert_eq!(request.aspect_ratio.as_deref(), Some("9:16"));
    assert_eq!(request.duration, Some(5));
    assert_eq!(
        request.image.as_deref(),
        Some("https://assets.example.com/start.png")
    );
    assert_eq!(
        request.image_tail.as_deref(),
        Some("https://assets.example.com/end.png")
    );
    assert_eq!(request.mode.as_deref(), Some("pro"));

    let serialized = serde_json::to_value(KlingVideoGenerationRequest {
        aspect_ratio: request.aspect_ratio,
        callback_url: request.callback_url,
        cfg_scale: request.cfg_scale,
        duration: request.duration,
        image: request.image,
        image_tail: request.image_tail,
        mode: request.mode,
        model: request.model,
        negative_prompt: request.negative_prompt,
        prompt: request.prompt,
    })
    .expect("generated SDK request should serialize");
    assert_eq!(
        serialized.get("prompt").and_then(|value| value.as_str()),
        Some("Neon launch clip")
    );
}

#[test]
fn maps_all_vidu_video_operations_to_generated_requests() {
    let image_plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Animate product hero".to_string(),
        negative_prompt: None,
        scene: "product_i2v".to_string(),
        provider_code: Some("vidu".to_string()),
        operation: Some("image_to_video".to_string()),
        model: Some("vidu2.0".to_string()),
        resolution: Some("720p".to_string()),
        aspect_ratio: Some("1:1".to_string()),
        duration_seconds: Some(4),
        start_image: Some("drive://spaces/space/nodes/start".to_string()),
        end_image: None,
        reference_images: vec![],
        motion_strength: Some("small".to_string()),
        webhook_url: None,
        idempotency_key: Some("idem-image".to_string()),
    })
    .expect("vidu image-to-video plan should build");
    assert_eq!(
        image_plan.provider_operation,
        VideoProviderOperation::ViduImageToVideo
    );
    let image_request = build_vidu_image_to_video_request(&image_plan);
    assert_eq!(
        image_request.images,
        vec!["drive://spaces/space/nodes/start".to_string()]
    );
    assert_eq!(
        image_request.prompt.as_deref(),
        Some("Animate product hero")
    );
    assert!(image_request
        .payload
        .as_deref()
        .expect("payload should carry trace fields")
        .contains("product_i2v"));

    let start_end_plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Morph between product states".to_string(),
        negative_prompt: None,
        scene: "product_start_end".to_string(),
        provider_code: Some("vidu".to_string()),
        operation: None,
        model: Some("vidu2.0".to_string()),
        resolution: Some("1080p".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(6),
        start_image: Some("drive://spaces/space/nodes/start".to_string()),
        end_image: Some("drive://spaces/space/nodes/end".to_string()),
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: Some("idem-start-end".to_string()),
    })
    .expect("vidu start-end plan should build");
    assert_eq!(
        start_end_plan.provider_operation,
        VideoProviderOperation::ViduStartEndToVideo
    );
    let start_end_request = build_vidu_start_end_to_video_request(&start_end_plan);
    assert_eq!(
        start_end_request.images,
        vec![
            "drive://spaces/space/nodes/start".to_string(),
            "drive://spaces/space/nodes/end".to_string(),
        ]
    );
    assert_eq!(
        start_end_request.prompt.as_deref(),
        Some("Morph between product states")
    );

    let reference_plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Reference driven clip".to_string(),
        negative_prompt: None,
        scene: "product_reference".to_string(),
        provider_code: Some("vidu".to_string()),
        operation: Some("reference_to_video".to_string()),
        model: Some("vidu2.0".to_string()),
        resolution: Some("1080p".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(8),
        start_image: None,
        end_image: None,
        reference_images: vec![
            "drive://spaces/space/nodes/ref-1".to_string(),
            "drive://spaces/space/nodes/ref-2".to_string(),
        ],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: Some("idem-reference".to_string()),
    })
    .expect("vidu reference plan should build");
    assert_eq!(
        reference_plan.provider_operation,
        VideoProviderOperation::ViduReferenceToVideo
    );
    let reference_request = build_vidu_reference_to_video_request(&reference_plan);
    assert_eq!(
        reference_request.images,
        vec![
            "drive://spaces/space/nodes/ref-1".to_string(),
            "drive://spaces/space/nodes/ref-2".to_string(),
        ]
    );
    assert_eq!(
        reference_request.prompt.as_deref(),
        Some("Reference driven clip")
    );
}

#[test]
fn maps_video_dispatch_plan_to_generated_volcengine_request() {
    let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "A launch teaser with dramatic lighting".to_string(),
        negative_prompt: None,
        scene: "launch_teaser".to_string(),
        provider_code: Some("volcengine".to_string()),
        operation: None,
        model: Some("doubao-seedance".to_string()),
        resolution: None,
        aspect_ratio: None,
        duration_seconds: None,
        start_image: None,
        end_image: None,
        reference_images: vec!["https://assets.example.com/ref.png".to_string()],
        motion_strength: None,
        webhook_url: Some("https://app.example.com/hooks/volc".to_string()),
        idempotency_key: Some("idem-volc-001".to_string()),
    })
    .expect("dispatch plan should build");

    let request = build_volcengine_video_generation_request(&plan);
    assert_eq!(request.model, "doubao-seedance");
    assert_eq!(
        request.callback_url.as_deref(),
        Some("https://app.example.com/hooks/volc")
    );
    assert_eq!(request.content.len(), 2);
    assert_eq!(request.content[0].r#type, "text");
    assert_eq!(
        request.content[0].text.as_deref(),
        Some("A launch teaser with dramatic lighting")
    );
    assert_eq!(request.content[1].r#type, "image_url");
    assert_eq!(
        request.content[1].image_url.as_deref(),
        Some("https://assets.example.com/ref.png")
    );
    let metadata = request
        .metadata
        .as_ref()
        .expect("volcengine metadata should carry trace fields");
    assert_eq!(
        metadata.get("provider").map(String::as_str),
        Some("volcengine")
    );
    assert_eq!(
        metadata.get("scene").map(String::as_str),
        Some("launch_teaser")
    );
    assert_eq!(
        metadata.get("idempotencyKey").map(String::as_str),
        Some("idem-volc-001")
    );

    let serialized = serde_json::to_value(VolcengineContentGenerationTaskCreateRequest {
        callback_url: request.callback_url,
        content: request.content,
        metadata: request.metadata,
        model: request.model,
    })
    .expect("generated SDK request should serialize");
    assert_eq!(
        serialized.get("model").and_then(|value| value.as_str()),
        Some("doubao-seedance")
    );
}

#[test]
fn maps_video_dispatch_plan_to_openai_compatible_generated_video_request() {
    let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "OpenAI-compatible video request".to_string(),
        negative_prompt: None,
        scene: "openai_video".to_string(),
        provider_code: Some("openai".to_string()),
        operation: None,
        model: Some("sora-video".to_string()),
        resolution: Some("1280x720".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(6),
        start_image: Some("https://assets.example.com/start.png".to_string()),
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: Some("idem-openai-001".to_string()),
    })
    .expect("openai-compatible dispatch plan should build");

    assert_eq!(
        plan.provider_operation,
        VideoProviderOperation::OpenAiVideoGeneration
    );
    assert_eq!(plan.claw_router_api_path, "/v1/videos");
    assert_eq!(plan.claw_router_sdk_resource, "video");
    assert_eq!(plan.claw_router_sdk_method, "create");

    let request = build_openai_video_create_request(&plan);
    assert_eq!(request.model, "sora-video");
    assert_eq!(request.prompt, "OpenAI-compatible video request");
    assert_eq!(
        request.image.as_deref(),
        Some("https://assets.example.com/start.png")
    );
    assert_eq!(request.seconds, Some(6));
    assert_eq!(request.size.as_deref(), Some("1280x720"));
    let metadata = request
        .metadata
        .as_ref()
        .expect("openai-compatible metadata should carry trace fields");
    assert_eq!(metadata.get("provider").map(String::as_str), Some("openai"));
    assert_eq!(
        metadata.get("scene").map(String::as_str),
        Some("openai_video")
    );
    assert_eq!(
        metadata.get("aspectRatio").map(String::as_str),
        Some("16:9")
    );
    assert_eq!(
        metadata.get("idempotencyKey").map(String::as_str),
        Some("idem-openai-001")
    );

    let serialized = serde_json::to_value(OpenAiVideoCreateRequest {
        image: request.image,
        metadata: request.metadata,
        model: request.model,
        prompt: request.prompt,
        seconds: request.seconds,
        size: request.size,
        video: request.video,
    })
    .expect("generated SDK request should serialize");
    assert_eq!(
        serialized.get("model").and_then(|value| value.as_str()),
        Some("sora-video")
    );
}

#[test]
fn keeps_openai_compatible_aspect_ratio_out_of_video_size_field() {
    let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "OpenAI-compatible video request with aspect ratio only".to_string(),
        negative_prompt: None,
        scene: "openai_aspect_ratio".to_string(),
        provider_code: Some("openai".to_string()),
        operation: None,
        model: Some("sora-video".to_string()),
        resolution: None,
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(6),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: Some("idem-openai-aspect".to_string()),
    })
    .expect("openai-compatible dispatch plan should build");

    let request = build_openai_video_create_request(&plan);

    assert_eq!(
        request.size, None,
        "OpenAI-compatible size must be a resolution/size value, not an aspect ratio"
    );
    let metadata = request
        .metadata
        .as_ref()
        .expect("openai-compatible metadata should carry aspect ratio");
    assert_eq!(
        metadata.get("aspectRatio").map(String::as_str),
        Some("16:9")
    );
}

#[test]
fn declares_video_provider_gateway_operation_support_without_raw_http_fallbacks() {
    let vidu = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Vidu task".to_string(),
        negative_prompt: None,
        scene: "support".to_string(),
        provider_code: Some("vidu".to_string()),
        operation: Some("text_to_video".to_string()),
        model: Some("vidu2.0".to_string()),
        resolution: None,
        aspect_ratio: None,
        duration_seconds: Some(4),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("vidu dispatch plan should build");
    let kling = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Kling task".to_string(),
        negative_prompt: None,
        scene: "support".to_string(),
        provider_code: Some("kling".to_string()),
        operation: None,
        model: Some("kling-v2".to_string()),
        resolution: None,
        aspect_ratio: None,
        duration_seconds: Some(5),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("kling dispatch plan should build");
    let provider_native = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Future provider task".to_string(),
        negative_prompt: None,
        scene: "support".to_string(),
        provider_code: Some("runway".to_string()),
        operation: None,
        model: Some("runway-gen3".to_string()),
        resolution: None,
        aspect_ratio: None,
        duration_seconds: Some(5),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("provider-native dispatch plan should build");
    let openai = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "OpenAI-compatible task".to_string(),
        negative_prompt: None,
        scene: "support".to_string(),
        provider_code: Some("openai".to_string()),
        operation: None,
        model: Some("sora-video".to_string()),
        resolution: None,
        aspect_ratio: None,
        duration_seconds: Some(5),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("openai-compatible dispatch plan should build");

    assert!(provider_gateway_supports_create_operation(&vidu));
    assert!(provider_gateway_supports_retrieve_operation(&vidu));
    assert!(provider_gateway_supports_create_operation(&kling));
    assert!(provider_gateway_supports_retrieve_operation(&kling));
    assert!(provider_gateway_supports_create_operation(&openai));
    assert!(provider_gateway_supports_retrieve_operation(&openai));
    assert_eq!(
        provider_native.provider_operation,
        VideoProviderOperation::ProviderNativeVideoGeneration
    );
    assert!(!provider_gateway_supports_create_operation(
        &provider_native
    ));
    assert!(!provider_gateway_supports_retrieve_operation(
        &provider_native
    ));
}
