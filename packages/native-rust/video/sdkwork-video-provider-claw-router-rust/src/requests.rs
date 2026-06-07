use std::collections::HashMap;

use clawrouter_open_sdk::{
    KlingVideoGenerationRequest, OpenAiVideoCreateRequest, ViduImageToVideoRequest,
    ViduReferenceToVideoRequest, ViduStartEndToVideoRequest, ViduTextToVideoRequest,
    VolcengineContentGenerationTaskCreateRequest, VolcengineContentPart,
};
use sdkwork_video_core::VideoProviderDispatchPlan;

pub fn build_vidu_text_to_video_request(
    plan: &VideoProviderDispatchPlan,
) -> ViduTextToVideoRequest {
    ViduTextToVideoRequest {
        aspect_ratio: plan.aspect_ratio.clone(),
        callback_url: plan.callback_url.clone(),
        duration: plan.duration_seconds.map(i64::from),
        model: model_or_provider(plan),
        movement_amplitude: plan.motion_strength.clone(),
        payload: provider_payload(plan),
        prompt: plan.prompt.clone(),
        resolution: plan.resolution.clone(),
        seed: None,
    }
}

pub fn build_vidu_image_to_video_request(
    plan: &VideoProviderDispatchPlan,
) -> ViduImageToVideoRequest {
    ViduImageToVideoRequest {
        aspect_ratio: plan.aspect_ratio.clone(),
        callback_url: plan.callback_url.clone(),
        duration: plan.duration_seconds.map(i64::from),
        images: plan.source_images.clone(),
        model: model_or_provider(plan),
        movement_amplitude: plan.motion_strength.clone(),
        payload: provider_payload(plan),
        prompt: Some(plan.prompt.clone()),
        resolution: plan.resolution.clone(),
        seed: None,
    }
}

pub fn build_vidu_start_end_to_video_request(
    plan: &VideoProviderDispatchPlan,
) -> ViduStartEndToVideoRequest {
    ViduStartEndToVideoRequest {
        aspect_ratio: plan.aspect_ratio.clone(),
        callback_url: plan.callback_url.clone(),
        duration: plan.duration_seconds.map(i64::from),
        images: plan.source_images.clone(),
        model: model_or_provider(plan),
        movement_amplitude: plan.motion_strength.clone(),
        payload: provider_payload(plan),
        prompt: Some(plan.prompt.clone()),
        resolution: plan.resolution.clone(),
        seed: None,
    }
}

pub fn build_vidu_reference_to_video_request(
    plan: &VideoProviderDispatchPlan,
) -> ViduReferenceToVideoRequest {
    ViduReferenceToVideoRequest {
        aspect_ratio: plan.aspect_ratio.clone(),
        callback_url: plan.callback_url.clone(),
        duration: plan.duration_seconds.map(i64::from),
        images: plan.source_images.clone(),
        model: model_or_provider(plan),
        movement_amplitude: plan.motion_strength.clone(),
        payload: provider_payload(plan),
        prompt: Some(plan.prompt.clone()),
        resolution: plan.resolution.clone(),
        seed: None,
    }
}

pub fn build_kling_video_generation_request(
    plan: &VideoProviderDispatchPlan,
) -> KlingVideoGenerationRequest {
    KlingVideoGenerationRequest {
        aspect_ratio: plan.aspect_ratio.clone(),
        callback_url: plan.callback_url.clone(),
        cfg_scale: None,
        duration: plan.duration_seconds.map(i64::from),
        image: plan
            .start_image
            .clone()
            .or_else(|| plan.source_images.first().cloned()),
        image_tail: plan.end_image.clone(),
        mode: plan.motion_strength.clone(),
        model: plan.model.clone(),
        negative_prompt: plan.negative_prompt.clone(),
        prompt: plan.prompt.clone(),
    }
}

pub fn build_volcengine_video_generation_request(
    plan: &VideoProviderDispatchPlan,
) -> VolcengineContentGenerationTaskCreateRequest {
    let mut content = vec![VolcengineContentPart {
        file_id: None,
        image_url: None,
        text: Some(plan.prompt.clone()),
        r#type: "text".to_string(),
        video_url: None,
    }];
    for image in &plan.source_images {
        content.push(VolcengineContentPart {
            file_id: None,
            image_url: Some(image.clone()),
            text: None,
            r#type: "image_url".to_string(),
            video_url: None,
        });
    }

    let mut metadata = HashMap::new();
    metadata.insert("provider".to_string(), plan.provider_code.clone());
    metadata.insert("scene".to_string(), plan.scene.clone());
    if let Some(aspect_ratio) = &plan.aspect_ratio {
        metadata.insert("aspectRatio".to_string(), aspect_ratio.clone());
    }
    if let Some(duration_seconds) = plan.duration_seconds {
        metadata.insert("durationSeconds".to_string(), duration_seconds.to_string());
    }
    if let Some(idempotency_key) = &plan.idempotency_key {
        metadata.insert("idempotencyKey".to_string(), idempotency_key.clone());
    }

    VolcengineContentGenerationTaskCreateRequest {
        callback_url: plan.callback_url.clone(),
        content,
        metadata: (!metadata.is_empty()).then_some(metadata),
        model: model_or_provider(plan),
    }
}

pub fn build_openai_video_create_request(
    plan: &VideoProviderDispatchPlan,
) -> OpenAiVideoCreateRequest {
    OpenAiVideoCreateRequest {
        image: plan
            .start_image
            .clone()
            .or_else(|| plan.source_images.first().cloned()),
        metadata: provider_metadata(plan),
        model: model_or_provider(plan),
        prompt: plan.prompt.clone(),
        seconds: plan.duration_seconds.map(i64::from),
        size: plan.resolution.clone(),
        video: None,
    }
}

fn provider_payload(plan: &VideoProviderDispatchPlan) -> Option<String> {
    let mut payload = serde_json::Map::new();
    payload.insert(
        "scene".to_string(),
        serde_json::Value::String(plan.scene.clone()),
    );
    if let Some(idempotency_key) = plan
        .idempotency_key
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        payload.insert(
            "idempotencyKey".to_string(),
            serde_json::Value::String(idempotency_key.to_string()),
        );
    }
    Some(serde_json::Value::Object(payload).to_string())
}

fn provider_metadata(plan: &VideoProviderDispatchPlan) -> Option<HashMap<String, String>> {
    let mut metadata = HashMap::new();
    metadata.insert("provider".to_string(), plan.provider_code.clone());
    metadata.insert("scene".to_string(), plan.scene.clone());
    if let Some(aspect_ratio) = &plan.aspect_ratio {
        metadata.insert("aspectRatio".to_string(), aspect_ratio.clone());
    }
    if let Some(duration_seconds) = plan.duration_seconds {
        metadata.insert("durationSeconds".to_string(), duration_seconds.to_string());
    }
    if let Some(idempotency_key) = &plan.idempotency_key {
        metadata.insert("idempotencyKey".to_string(), idempotency_key.clone());
    }
    (!metadata.is_empty()).then_some(metadata)
}

fn model_or_provider(plan: &VideoProviderDispatchPlan) -> String {
    plan.model
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(&plan.provider_code)
        .to_string()
}
