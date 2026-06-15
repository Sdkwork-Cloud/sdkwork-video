use sdkwork_drive_product::uploader::{UploaderActor, UploaderRetention, UploaderTarget};
use sdkwork_video_core::{
    build_drive_uploader_command_for_generated_video_output,
    normalize_provider_task_video_generation_result, plan_drive_import_for_generated_video_outputs,
    plan_video_generation_provider_dispatch, DriveGeneratedVideoContext, GeneratedVideoOutput,
    ProviderGeneratedVideoAsset, ProviderTaskErrorSnapshot, ProviderTaskSnapshot,
    VideoGenerationActor, VideoGenerationCreateCommand, VideoGenerationRuntimeStatus,
    VideoProviderOperation, VideoProviderTaskMode, GENERATED_VIDEO_DEFAULT_CHUNK_SIZE_BYTES,
    VIDEO_CAPABILITY, VIDEO_DOMAIN, VIDEO_WORKSPACE,
};

#[test]
fn keeps_lib_rs_as_public_module_boundary() {
    let lib_rs = include_str!("../src/lib.rs");

    assert!(
        lib_rs.lines().count() <= 80,
        "sdkwork-video-core-rust src/lib.rs must stay a small module assembly boundary"
    );
    for forbidden in [
        "pub struct ",
        "pub enum ",
        "pub fn ",
        "impl ",
        "use sdkwork_drive_product",
    ] {
        assert!(
            !lib_rs.contains(forbidden),
            "src/lib.rs must not contain authored business logic marker {forbidden}"
        );
    }
    assert!(lib_rs.contains("pub mod "));
    assert!(lib_rs.contains("pub use "));
}

#[test]
fn exposes_video_domain_identity() {
    assert_eq!(VIDEO_WORKSPACE, "sdkwork-video");
    assert_eq!(VIDEO_DOMAIN, "content");
    assert_eq!(VIDEO_CAPABILITY, "video");
}

#[test]
fn plans_video_generation_provider_dispatch_through_claw_router_sdk_boundary() {
    let vidu = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Cinematic product reveal".to_string(),
        negative_prompt: Some("jitter, flicker".to_string()),
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
        idempotency_key: Some("video-idem-001".to_string()),
    })
    .expect("vidu dispatch plan should build");

    assert_eq!(vidu.provider_code, "vidu");
    assert_eq!(
        vidu.provider_operation,
        VideoProviderOperation::ViduTextToVideo
    );
    assert_eq!(vidu.task_mode, VideoProviderTaskMode::Task);
    assert_eq!(vidu.claw_router_api_path, "/vidu/ent/v2/text2video");
    assert_eq!(vidu.claw_router_sdk_resource, "videos_vidu");
    assert_eq!(vidu.claw_router_sdk_method, "create_ent_v2_text2video");
    assert_eq!(vidu.duration_seconds, Some(8));
    assert_eq!(vidu.aspect_ratio.as_deref(), Some("16:9"));
    assert_eq!(
        vidu.callback_url.as_deref(),
        Some("https://app.example.com/hooks/vidu")
    );

    let kling = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "Slow camera move through neon signage".to_string(),
        negative_prompt: None,
        scene: "campaign_clip".to_string(),
        provider_code: Some("kling".to_string()),
        operation: None,
        model: Some("kling-v2".to_string()),
        resolution: None,
        aspect_ratio: Some("9:16".to_string()),
        duration_seconds: Some(5),
        start_image: Some("drive://spaces/space/nodes/start".to_string()),
        end_image: None,
        reference_images: vec![],
        motion_strength: Some("pro".to_string()),
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("kling dispatch plan should build");

    assert_eq!(
        kling.provider_operation,
        VideoProviderOperation::KlingVideoGeneration
    );
    assert_eq!(kling.claw_router_api_path, "/kling/v1/videos/generations");
    assert_eq!(kling.claw_router_sdk_resource, "videos_kling");
    assert_eq!(kling.claw_router_sdk_method, "create_v1_videos_generation");
    assert_eq!(
        kling.source_images,
        vec!["drive://spaces/space/nodes/start".to_string()]
    );

    let volcengine = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
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
        idempotency_key: None,
    })
    .expect("volcengine dispatch plan should build");

    assert_eq!(
        volcengine.provider_operation,
        VideoProviderOperation::VolcengineContentGeneration
    );
    assert_eq!(
        volcengine.claw_router_api_path,
        "/volcengine/api/v3/contents/generations/tasks"
    );
    assert_eq!(volcengine.claw_router_sdk_resource, "videos_volcengine");
    assert_eq!(
        volcengine.claw_router_sdk_method,
        "create_api_v3_contents_generations_task"
    );

    let openai = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
        prompt: "A short cinematic scene generated by an OpenAI-compatible video model".to_string(),
        negative_prompt: None,
        scene: "openai_video".to_string(),
        provider_code: Some("openai".to_string()),
        operation: None,
        model: Some("sora-video".to_string()),
        resolution: Some("1280x720".to_string()),
        aspect_ratio: None,
        duration_seconds: Some(6),
        start_image: Some("https://assets.example.com/openai-start.png".to_string()),
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: Some("idem-openai-001".to_string()),
    })
    .expect("openai-compatible dispatch plan should build");

    assert_eq!(
        openai.provider_operation,
        VideoProviderOperation::OpenAiVideoGeneration
    );
    assert_eq!(openai.task_mode, VideoProviderTaskMode::Task);
    assert_eq!(openai.claw_router_api_path, "/v1/videos");
    assert_eq!(openai.claw_router_sdk_resource, "video");
    assert_eq!(openai.claw_router_sdk_method, "create");
}

#[test]
fn rejects_vidu_image_to_video_without_source_image() {
    let error = plan_video_generation_provider_dispatch(&vidu_command(
        "image_to_video",
        None,
        None,
        vec![],
    ))
    .expect_err("vidu image_to_video must require one source image");

    assert_eq!(error, "vidu image_to_video requires one source image");
}

#[test]
fn rejects_vidu_start_end_to_video_without_start_and_end_images() {
    let error = plan_video_generation_provider_dispatch(&vidu_command(
        "start_end_to_video",
        Some("drive://spaces/space/nodes/start"),
        None,
        vec![],
    ))
    .expect_err("vidu start_end_to_video must require start and end images");

    assert_eq!(
        error,
        "vidu start_end_to_video requires start and end images"
    );
}

#[test]
fn rejects_vidu_start_end_to_video_when_reference_image_replaces_end_image() {
    let error = plan_video_generation_provider_dispatch(&vidu_command(
        "start_end_to_video",
        Some("drive://spaces/space/nodes/start"),
        None,
        vec!["drive://spaces/space/nodes/ref-1"],
    ))
    .expect_err("vidu start_end_to_video must require an explicit end image");

    assert_eq!(
        error,
        "vidu start_end_to_video requires start and end images"
    );
}

#[test]
fn rejects_vidu_reference_to_video_without_reference_images() {
    let error = plan_video_generation_provider_dispatch(&vidu_command(
        "reference_to_video",
        None,
        None,
        vec![],
    ))
    .expect_err("vidu reference_to_video must require at least one reference image");

    assert_eq!(
        error,
        "vidu reference_to_video requires at least one reference image"
    );
}

#[test]
fn rejects_vidu_reference_to_video_when_start_image_replaces_reference_image() {
    let error = plan_video_generation_provider_dispatch(&vidu_command(
        "reference_to_video",
        Some("drive://spaces/space/nodes/start"),
        None,
        vec![],
    ))
    .expect_err("vidu reference_to_video must require reference images");

    assert_eq!(
        error,
        "vidu reference_to_video requires at least one reference image"
    );
}

#[test]
fn normalizes_video_provider_task_result_for_polling_or_webhook_consistency() {
    let normalized = normalize_provider_task_video_generation_result(
        "kling",
        ProviderTaskSnapshot {
            task_id: Some("task-kling-001".to_string()),
            id: Some("provider-video-001".to_string()),
            status: Some("SUCCEEDED".to_string()),
            state: Some("completed".to_string()),
            model: Some("kling-v2".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-0".to_string()),
                uri: Some("provider://kling/tasks/task-kling-001/videos/0".to_string()),
                url: Some("https://provider.example.com/kling/0.mp4".to_string()),
                mime_type: Some("video/mp4".to_string()),
                width: Some(1280),
                height: Some(720),
                duration_seconds: Some(5),
            }],
            error: None,
        },
    )
    .expect("provider task should normalize");

    assert_eq!(normalized.provider_code, "kling");
    assert_eq!(
        normalized.provider_task_id.as_deref(),
        Some("task-kling-001")
    );
    assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Importing);
    assert!(normalized.provider_terminal);
    assert!(normalized.ready_for_drive_import);
    assert_eq!(normalized.outputs.len(), 1);
    assert_eq!(
        normalized.outputs[0].mime_type.as_deref(),
        Some("video/mp4")
    );
    assert_eq!(normalized.outputs[0].duration_seconds, Some(5));
}

#[test]
fn infers_provider_video_mime_type_and_file_extension_from_signed_url() {
    let normalized = normalize_provider_task_video_generation_result(
        "kling",
        ProviderTaskSnapshot {
            task_id: Some("task-kling-webm".to_string()),
            id: None,
            status: Some("succeeded".to_string()),
            state: None,
            model: Some("kling-v2".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-webm".to_string()),
                uri: None,
                url: Some("https://provider.example.com/kling-output.webm?token=abc".to_string()),
                mime_type: None,
                width: Some(1280),
                height: Some(720),
                duration_seconds: Some(5),
            }],
            error: None,
        },
    )
    .expect("signed provider video URL should normalize");

    assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Importing);
    assert_eq!(
        normalized.outputs[0].mime_type.as_deref(),
        Some("video/webm")
    );
    assert_eq!(
        normalized.outputs[0].file_name.as_deref(),
        Some("generated-0.webm")
    );
}

#[test]
fn normalizes_provider_specific_video_status_aliases() {
    let succeed = normalize_provider_task_video_generation_result(
        "kling",
        ProviderTaskSnapshot {
            task_id: Some("task-kling-succeed".to_string()),
            id: None,
            status: Some("succeed".to_string()),
            state: None,
            model: Some("kling-v2".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-succeed".to_string()),
                uri: None,
                url: Some("https://provider.example.com/kling-succeed.mp4".to_string()),
                mime_type: Some("video/mp4".to_string()),
                width: Some(1280),
                height: Some(720),
                duration_seconds: Some(5),
            }],
            error: None,
        },
    )
    .expect("kling succeed status should normalize");

    assert_eq!(succeed.status, VideoGenerationRuntimeStatus::Importing);
    assert!(succeed.provider_terminal);
    assert!(succeed.ready_for_drive_import);
    assert_eq!(succeed.outputs.len(), 1);

    let created = normalize_provider_task_video_generation_result(
        "volcengine",
        ProviderTaskSnapshot {
            task_id: Some("task-volc-created".to_string()),
            id: None,
            status: Some("created".to_string()),
            state: None,
            model: Some("doubao-seedance".to_string()),
            videos: vec![],
            error: None,
        },
    )
    .expect("volcengine created status should normalize");

    assert_eq!(created.status, VideoGenerationRuntimeStatus::Submitted);
    assert!(!created.provider_terminal);
    assert!(!created.ready_for_drive_import);

    let queueing = normalize_provider_task_video_generation_result(
        "vidu",
        ProviderTaskSnapshot {
            task_id: Some("task-vidu-queueing".to_string()),
            id: None,
            status: None,
            state: Some("queueing".to_string()),
            model: Some("vidu2.0".to_string()),
            videos: vec![],
            error: None,
        },
    )
    .expect("vidu queueing state should normalize");

    assert_eq!(queueing.status, VideoGenerationRuntimeStatus::Submitted);
    assert!(!queueing.provider_terminal);
    assert!(!queueing.ready_for_drive_import);
}

#[test]
fn treats_provider_outputs_without_status_as_ready_for_drive_import() {
    let normalized = normalize_provider_task_video_generation_result(
        "openai",
        ProviderTaskSnapshot {
            task_id: Some("video-with-output-no-status".to_string()),
            id: None,
            status: None,
            state: None,
            model: Some("sora-video".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-no-status".to_string()),
                uri: None,
                url: Some("https://provider.example.com/no-status.mp4".to_string()),
                mime_type: None,
                width: None,
                height: None,
                duration_seconds: Some(6),
            }],
            error: None,
        },
    )
    .expect("provider outputs without explicit status should normalize");

    assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Importing);
    assert!(normalized.provider_terminal);
    assert!(normalized.ready_for_drive_import);
    assert_eq!(normalized.outputs.len(), 1);
}

#[test]
fn normalizes_video_provider_failure_without_drive_import_outputs() {
    let normalized = normalize_provider_task_video_generation_result(
        "vidu",
        ProviderTaskSnapshot {
            task_id: Some("task-vidu-001".to_string()),
            id: None,
            status: Some("failed".to_string()),
            state: None,
            model: Some("vidu2.0".to_string()),
            videos: vec![],
            error: Some(ProviderTaskErrorSnapshot {
                code: Some("provider_failed".to_string()),
                message: Some("Provider rejected prompt".to_string()),
                error_type: Some("moderation".to_string()),
            }),
        },
    )
    .expect("provider failure should normalize");

    assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Failed);
    assert!(normalized.provider_terminal);
    assert!(!normalized.ready_for_drive_import);
    assert!(normalized.outputs.is_empty());
    assert_eq!(normalized.error_code.as_deref(), Some("provider_failed"));
}

#[test]
fn plans_generated_videos_into_drive_ai_generated_space() {
    let plans = plan_drive_import_for_generated_video_outputs(
        DriveGeneratedVideoContext {
            tenant_id: "tenant-1".to_string(),
            organization_id: Some("org-1".to_string()),
            generation_id: "video-generation-001".to_string(),
            provider_code: "vidu".to_string(),
            model: Some("vidu2.0".to_string()),
            scene: "product_reveal".to_string(),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        vec![GeneratedVideoOutput {
            output_index: 0,
            provider_asset_id: Some("asset-0".to_string()),
            provider_uri: Some("provider://vidu/tasks/task-001/videos/0".to_string()),
            provider_url: Some("https://provider.example.com/video.mp4".to_string()),
            file_name: Some("product-reveal.mp4".to_string()),
            mime_type: Some("video/mp4".to_string()),
            size_bytes: Some(4096),
            width: Some(1920),
            height: Some(1080),
            duration_seconds: Some(8),
        }],
    )
    .expect("drive import plan should build");

    assert_eq!(plans.len(), 1);
    let plan = &plans[0];
    assert_eq!(plan.drive_space_type, "ai_generated");
    assert_eq!(plan.drive_owner_subject_type, "user");
    assert_eq!(plan.drive_upload_profile_code, "video");
    assert_eq!(plan.media_resource.kind, "video");
    assert_eq!(plan.media_resource.source, "drive");
    assert_eq!(plan.media_resource.url, None);
    assert_eq!(plan.media_resource.ai.provenance, "generated");
    assert_eq!(plan.media_resource.ai.provider.as_deref(), Some("vidu"));
    assert_eq!(plan.media_resource.ai.model.as_deref(), Some("vidu2.0"));
    assert_eq!(
        plan.media_resource
            .metadata
            .get("scene")
            .map(String::as_str),
        Some("product_reveal")
    );
}

#[test]
fn builds_drive_uploader_command_for_generated_video_output() {
    let plans = plan_drive_import_for_generated_video_outputs(
        DriveGeneratedVideoContext {
            tenant_id: "tenant-1".to_string(),
            organization_id: None,
            generation_id: "video-generation-001".to_string(),
            provider_code: "kling".to_string(),
            model: Some("kling-v2".to_string()),
            scene: "campaign_clip".to_string(),
            actor: VideoGenerationActor::Anonymous {
                anonymous_id: "anon-001".to_string(),
            },
        },
        vec![GeneratedVideoOutput {
            output_index: 0,
            provider_asset_id: None,
            provider_uri: Some("provider://kling/tasks/task-001/videos/0".to_string()),
            provider_url: None,
            file_name: Some("campaign.mp4".to_string()),
            mime_type: Some("video/mp4".to_string()),
            size_bytes: Some(8192),
            width: Some(1280),
            height: Some(720),
            duration_seconds: Some(5),
        }],
    )
    .expect("drive import plan should build");

    let command = build_drive_uploader_command_for_generated_video_output(
        &plans[0],
        "tenant-1",
        None,
        "operator-001",
        1_780_000_000_000,
    )
    .expect("drive uploader command should build");

    assert_eq!(command.app_id, VIDEO_WORKSPACE);
    assert_eq!(command.app_resource_type, "ai_video_generation_output");
    assert_eq!(command.app_resource_id, "video-generation-001:0");
    assert_eq!(command.scene.as_deref(), Some("campaign_clip"));
    assert_eq!(command.source.as_deref(), Some("ai_generated"));
    assert_eq!(command.upload_profile_code, "video");
    assert_eq!(command.original_file_name, "campaign.mp4");
    assert_eq!(command.content_type, "video/mp4");
    assert_eq!(command.content_length, 8192);
    assert_eq!(
        command.chunk_size_bytes,
        GENERATED_VIDEO_DEFAULT_CHUNK_SIZE_BYTES
    );
    assert!(matches!(
        command.actor,
        UploaderActor::Anonymous { ref anonymous_id } if anonymous_id == "anon-001"
    ));
    assert!(matches!(
        command.target,
        UploaderTarget::AiGeneratedSpace {
            parent_node_id: None
        }
    ));
    assert!(matches!(command.retention, UploaderRetention::LongTerm));
}

fn vidu_command(
    operation: &str,
    start_image: Option<&str>,
    end_image: Option<&str>,
    reference_images: Vec<&str>,
) -> VideoGenerationCreateCommand {
    VideoGenerationCreateCommand {
        prompt: "Cinematic product reveal".to_string(),
        negative_prompt: None,
        scene: "product_reveal".to_string(),
        provider_code: Some("vidu".to_string()),
        operation: Some(operation.to_string()),
        model: Some("vidu2.0".to_string()),
        resolution: Some("1080p".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(8),
        start_image: start_image.map(str::to_string),
        end_image: end_image.map(str::to_string),
        reference_images: reference_images.into_iter().map(str::to_string).collect(),
        motion_strength: Some("normal".to_string()),
        webhook_url: None,
        idempotency_key: Some("video-idem-validation".to_string()),
    }
}
