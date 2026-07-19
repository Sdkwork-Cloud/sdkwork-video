use crate::{
    models::{
        VideoGenerationCommand, VideoGenerationCreateCommand, VideoGenerationModelSelection,
        VideoProviderDispatchPlan, VideoVendorId,
    },
    status::{VideoProviderOperation, VideoProviderTaskMode},
    text::{
        normalize_operation_code, normalize_provider_code_for_storage, normalized_optional_text,
        require_trimmed, validate_scene_code,
    },
};

pub fn plan_video_generation_provider_dispatch(
    command: &VideoGenerationCreateCommand,
) -> Result<VideoProviderDispatchPlan, &'static str> {
    let vendor = command
        .provider_code
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("vidu");
    let model = match command.model.as_deref().map(str::trim) {
        Some(value) if !value.is_empty() => VideoGenerationModelSelection::named(value)?,
        _ => VideoGenerationModelSelection::VendorDefault,
    };
    plan_unified_video_generation_provider_dispatch(&VideoGenerationCommand {
        vendor: VideoVendorId::new(vendor)?,
        operation: command.operation.clone(),
        model,
        prompt: command.prompt.clone(),
        negative_prompt: command.negative_prompt.clone(),
        scene: command.scene.clone(),
        resolution: command.resolution.clone(),
        aspect_ratio: command.aspect_ratio.clone(),
        duration_seconds: command.duration_seconds,
        start_image: command.start_image.clone(),
        end_image: command.end_image.clone(),
        reference_images: command.reference_images.clone(),
        motion_strength: command.motion_strength.clone(),
        callback_url: command.webhook_url.clone(),
        idempotency_key: command.idempotency_key.clone(),
        vendor_parameters: None,
    })
}

pub fn plan_unified_video_generation_provider_dispatch(
    command: &VideoGenerationCommand,
) -> Result<VideoProviderDispatchPlan, &'static str> {
    let prompt = require_trimmed(&command.prompt, "video generation prompt is required")?;
    let scene = require_trimmed(&command.scene, "video generation scene is required")?;
    validate_scene_code(scene)?;

    let provider_code = normalize_provider_code_for_storage(command.vendor.as_str());
    let operation = command
        .operation
        .as_deref()
        .map(normalize_operation_code)
        .unwrap_or_default();
    let callback_url = normalized_optional_text(command.callback_url.as_deref());
    let start_image = normalized_optional_text(command.start_image.as_deref());
    let end_image = normalized_optional_text(command.end_image.as_deref());
    let reference_image_count = normalized_reference_image_count(&command.reference_images);
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

    let (provider_operation, task_mode) = match provider_code.as_str() {
        "vidu" => vidu_operation_for_command(&operation, &source_images),
        "kling" => (
            VideoProviderOperation::KlingVideoGeneration,
            VideoProviderTaskMode::Task,
        ),
        "volcengine" | "doubao" | "ark" => (
            VideoProviderOperation::VolcengineContentGeneration,
            VideoProviderTaskMode::Task,
        ),
        "openai" | "sora" | "openai-compatible" => (
            VideoProviderOperation::OpenAiVideoGeneration,
            VideoProviderTaskMode::Task,
        ),
        _ => return Err("video generation vendor is not supported by a registered provider"),
    };

    if let Some(duration_seconds) = command.duration_seconds {
        if duration_seconds <= 0 {
            return Err("video generation duration_seconds must be greater than 0");
        }
    }

    Ok(VideoProviderDispatchPlan {
        provider_id: String::new(),
        provider_code,
        provider_operation,
        task_mode,
        scene: scene.to_string(),
        prompt: prompt.to_string(),
        negative_prompt: normalized_optional_text(command.negative_prompt.as_deref()),
        model: command.model.as_named().map(str::to_string),
        resolution: normalized_optional_text(command.resolution.as_deref()),
        aspect_ratio: normalized_optional_text(command.aspect_ratio.as_deref()),
        duration_seconds: command.duration_seconds,
        start_image,
        end_image,
        source_images,
        motion_strength: normalized_optional_text(command.motion_strength.as_deref()),
        callback_url,
        idempotency_key: normalized_optional_text(command.idempotency_key.as_deref()),
        vendor_parameters: command.vendor_parameters.clone(),
    })
}

fn vidu_operation_for_command(
    operation: &str,
    source_images: &[String],
) -> (VideoProviderOperation, VideoProviderTaskMode) {
    if matches!(operation, "start_end_to_video" | "start_end2video")
        || (source_images.len() >= 2 && operation.is_empty())
    {
        return (
            VideoProviderOperation::ViduStartEndToVideo,
            VideoProviderTaskMode::Task,
        );
    }
    if matches!(operation, "image_to_video" | "img2video")
        || (source_images.len() == 1 && operation.is_empty())
    {
        return (
            VideoProviderOperation::ViduImageToVideo,
            VideoProviderTaskMode::Task,
        );
    }
    if matches!(operation, "reference_to_video" | "reference2video") {
        return (
            VideoProviderOperation::ViduReferenceToVideo,
            VideoProviderTaskMode::Task,
        );
    }
    (
        VideoProviderOperation::ViduTextToVideo,
        VideoProviderTaskMode::Task,
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

fn normalized_reference_image_count(reference_images: &[String]) -> usize {
    reference_images
        .iter()
        .filter(|image| normalized_optional_text(Some(image.as_str())).is_some())
        .count()
}

fn normalized_source_images(command: &VideoGenerationCommand) -> Vec<String> {
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
