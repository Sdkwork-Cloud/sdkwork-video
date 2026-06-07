use crate::{
    models::{VideoGenerationCreateCommand, VideoProviderDispatchPlan},
    status::{VideoProviderOperation, VideoProviderTaskMode},
    text::{
        normalize_operation_code, normalize_provider_code_for_storage, normalized_optional_text,
        require_trimmed, validate_scene_code,
    },
};

pub fn plan_video_generation_provider_dispatch(
    command: &VideoGenerationCreateCommand,
) -> Result<VideoProviderDispatchPlan, &'static str> {
    let prompt = require_trimmed(&command.prompt, "video generation prompt is required")?;
    let scene = require_trimmed(&command.scene, "video generation scene is required")?;
    validate_scene_code(scene)?;

    let provider_code = command
        .provider_code
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("vidu")
        .to_ascii_lowercase();
    let provider_code = normalize_provider_code_for_storage(&provider_code);
    let operation = command
        .operation
        .as_deref()
        .map(normalize_operation_code)
        .unwrap_or_default();
    let callback_url = normalized_optional_text(command.webhook_url.as_deref());
    let start_image = normalized_optional_text(command.start_image.as_deref());
    let end_image = normalized_optional_text(command.end_image.as_deref());
    let reference_image_count = normalized_reference_image_count(command);
    let source_images = normalized_source_images(command);
    if provider_code == "vidu" {
        validate_vidu_operation_inputs(
            &operation,
            start_image.as_deref(),
            end_image.as_deref(),
            reference_image_count,
            &source_images,
        )?;
    }

    let (
        provider_operation,
        task_mode,
        claw_router_api_path,
        claw_router_sdk_resource,
        claw_router_sdk_method,
    ) = match provider_code.as_str() {
        "vidu" => vidu_operation_for_command(&operation, &source_images),
        "kling" => (
            VideoProviderOperation::KlingVideoGeneration,
            VideoProviderTaskMode::Task,
            "/kling/v1/videos/generations",
            "videos_kling",
            "create_v1_videos_generation",
        ),
        "volcengine" | "doubao" | "ark" => (
            VideoProviderOperation::VolcengineContentGeneration,
            VideoProviderTaskMode::Task,
            "/volcengine/api/v3/contents/generations/tasks",
            "videos_volcengine",
            "create_api_v3_contents_generations_task",
        ),
        "openai" | "sora" | "openai-compatible" | "claw-router" | "clawrouter" => (
            VideoProviderOperation::OpenAiVideoGeneration,
            VideoProviderTaskMode::Task,
            "/v1/videos",
            "video",
            "create",
        ),
        _ => (
            VideoProviderOperation::ProviderNativeVideoGeneration,
            if callback_url.is_some() {
                VideoProviderTaskMode::Webhook
            } else {
                VideoProviderTaskMode::Task
            },
            "/v1/videos/generations",
            "video",
            "create",
        ),
    };

    if let Some(duration_seconds) = command.duration_seconds {
        if duration_seconds <= 0 {
            return Err("video generation duration_seconds must be greater than 0");
        }
    }

    Ok(VideoProviderDispatchPlan {
        provider_code,
        provider_operation,
        task_mode,
        claw_router_api_path,
        claw_router_sdk_resource,
        claw_router_sdk_method,
        scene: scene.to_string(),
        prompt: prompt.to_string(),
        negative_prompt: normalized_optional_text(command.negative_prompt.as_deref()),
        model: normalized_optional_text(command.model.as_deref()),
        resolution: normalized_optional_text(command.resolution.as_deref()),
        aspect_ratio: normalized_optional_text(command.aspect_ratio.as_deref()),
        duration_seconds: command.duration_seconds,
        start_image,
        end_image,
        source_images,
        motion_strength: normalized_optional_text(command.motion_strength.as_deref()),
        callback_url,
        idempotency_key: normalized_optional_text(command.idempotency_key.as_deref()),
    })
}

fn vidu_operation_for_command(
    operation: &str,
    source_images: &[String],
) -> (
    VideoProviderOperation,
    VideoProviderTaskMode,
    &'static str,
    &'static str,
    &'static str,
) {
    if matches!(operation, "start_end_to_video" | "start_end2video")
        || (source_images.len() >= 2 && operation.is_empty())
    {
        return (
            VideoProviderOperation::ViduStartEndToVideo,
            VideoProviderTaskMode::Task,
            "/vidu/ent/v2/start-end2video",
            "videos_vidu",
            "create_ent_v2_start_end2video",
        );
    }
    if matches!(operation, "image_to_video" | "img2video")
        || (source_images.len() == 1 && operation.is_empty())
    {
        return (
            VideoProviderOperation::ViduImageToVideo,
            VideoProviderTaskMode::Task,
            "/vidu/ent/v2/img2video",
            "videos_vidu",
            "create_ent_v2_img2video",
        );
    }
    if matches!(operation, "reference_to_video" | "reference2video") {
        return (
            VideoProviderOperation::ViduReferenceToVideo,
            VideoProviderTaskMode::Task,
            "/vidu/ent/v2/reference2video",
            "videos_vidu",
            "create_ent_v2_reference2video",
        );
    }
    (
        VideoProviderOperation::ViduTextToVideo,
        VideoProviderTaskMode::Task,
        "/vidu/ent/v2/text2video",
        "videos_vidu",
        "create_ent_v2_text2video",
    )
}

fn validate_vidu_operation_inputs(
    operation: &str,
    start_image: Option<&str>,
    end_image: Option<&str>,
    reference_image_count: usize,
    source_images: &[String],
) -> Result<(), &'static str> {
    if matches!(operation, "image_to_video" | "img2video") && source_images.is_empty() {
        return Err("vidu image_to_video requires one source image");
    }
    if matches!(operation, "start_end_to_video" | "start_end2video")
        && (start_image.is_none() || end_image.is_none())
    {
        return Err("vidu start_end_to_video requires start and end images");
    }
    if matches!(operation, "reference_to_video" | "reference2video") && reference_image_count == 0 {
        return Err("vidu reference_to_video requires at least one reference image");
    }
    Ok(())
}

fn normalized_reference_image_count(command: &VideoGenerationCreateCommand) -> usize {
    command
        .reference_images
        .iter()
        .filter(|image| normalized_optional_text(Some(image.as_str())).is_some())
        .count()
}

fn normalized_source_images(command: &VideoGenerationCreateCommand) -> Vec<String> {
    let mut images = Vec::new();
    if let Some(start_image) = normalized_optional_text(command.start_image.as_deref()) {
        images.push(start_image);
    }
    if let Some(end_image) = normalized_optional_text(command.end_image.as_deref()) {
        images.push(end_image);
    }
    for image in &command.reference_images {
        if let Some(image) = normalized_optional_text(Some(image.as_str())) {
            images.push(image);
        }
    }
    images
}
