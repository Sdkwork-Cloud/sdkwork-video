use clawrouter_open_sdk::{
    KlingVideoGenerationTask, OpenAiVideo, ProviderGeneratedMedia, ProviderTaskError, SdkworkError,
    ViduCreation, ViduTaskCreationsResponse, ViduVideoGenerationTask,
    VolcengineContentGenerationTask, VolcengineContentGenerationTaskCreateResponse,
    VolcengineContentPart,
};
use sdkwork_video_core::{
    normalize_provider_task_video_generation_result, NormalizedProviderVideoGenerationResult,
    ProviderGeneratedVideoAsset, ProviderTaskErrorSnapshot, ProviderTaskSnapshot,
    VideoProviderDispatchPlan,
};

pub(crate) fn normalize_kling_generation_task(
    plan: &VideoProviderDispatchPlan,
    task: KlingVideoGenerationTask,
) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
    normalize_provider_task_video_generation_result(
        &plan.provider_code,
        ProviderTaskSnapshot {
            task_id: task.task_id,
            id: task.id,
            status: task.status,
            state: task.state,
            model: task.model,
            videos: provider_assets_from_generated_media(task.videos.unwrap_or_default()),
            error: task.error.map(provider_error_from_claw_router),
        },
    )
    .map_err(sdk_error_from_normalization)
}

pub(crate) fn normalize_vidu_video_generation_task(
    plan: &VideoProviderDispatchPlan,
    task: ViduVideoGenerationTask,
) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
    normalize_provider_task_video_generation_result(
        &plan.provider_code,
        ProviderTaskSnapshot {
            task_id: task.task_id,
            id: None,
            status: task.state.clone(),
            state: task.state,
            model: task.model,
            videos: provider_assets_from_vidu_creations(task.creations.unwrap_or_default()),
            error: None,
        },
    )
    .map_err(sdk_error_from_normalization)
}

pub(crate) fn normalize_vidu_task_creations_response(
    plan: &VideoProviderDispatchPlan,
    task: ViduTaskCreationsResponse,
) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
    normalize_provider_task_video_generation_result(
        &plan.provider_code,
        ProviderTaskSnapshot {
            task_id: task.task_id,
            id: None,
            status: task.state.clone(),
            state: task.state,
            model: task.model,
            videos: provider_assets_from_vidu_creations(task.creations.unwrap_or_default()),
            error: None,
        },
    )
    .map_err(sdk_error_from_normalization)
}

pub(crate) fn normalize_openai_video(
    plan: &VideoProviderDispatchPlan,
    video: OpenAiVideo,
) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
    let video_id = video.id;
    let video_url = video.url.or(video.content_url);
    let videos = video_url
        .map(|url| ProviderGeneratedVideoAsset {
            id: Some(video_id.clone()),
            uri: Some(format!(
                "provider://{}/videos/{}",
                plan.provider_code, video_id
            )),
            url: Some(url),
            mime_type: None,
            width: None,
            height: None,
            duration_seconds: video.seconds.and_then(|value| i32::try_from(value).ok()),
        })
        .into_iter()
        .collect();

    normalize_provider_task_video_generation_result(
        &plan.provider_code,
        ProviderTaskSnapshot {
            task_id: Some(video_id.clone()),
            id: Some(video_id),
            status: Some(video.status),
            state: None,
            model: video.model,
            videos,
            error: None,
        },
    )
    .map_err(sdk_error_from_normalization)
}

pub(crate) fn normalize_volcengine_create_response(
    plan: &VideoProviderDispatchPlan,
    task: VolcengineContentGenerationTaskCreateResponse,
) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
    normalize_provider_task_video_generation_result(
        &plan.provider_code,
        ProviderTaskSnapshot {
            task_id: task.task_id,
            id: task.id,
            status: task.status,
            state: None,
            model: plan.model.clone(),
            videos: vec![],
            error: None,
        },
    )
    .map_err(sdk_error_from_normalization)
}

pub(crate) fn normalize_volcengine_generation_task(
    plan: &VideoProviderDispatchPlan,
    task: VolcengineContentGenerationTask,
) -> Result<NormalizedProviderVideoGenerationResult, SdkworkError> {
    let VolcengineContentGenerationTask {
        content,
        error,
        id,
        model,
        result,
        state,
        status,
        task_id,
        videos,
        ..
    } = task;
    let mut videos = videos.unwrap_or_default();
    videos.extend(provider_media_from_volcengine_content_parts(
        content.unwrap_or_default(),
    ));
    let result_status = result.as_ref().and_then(|result| result.status.clone());
    if let Some(result) = result {
        videos.extend(result.videos.unwrap_or_default());
        videos.extend(provider_media_from_volcengine_content_parts(
            result.content.unwrap_or_default(),
        ));
    }

    normalize_provider_task_video_generation_result(
        &plan.provider_code,
        ProviderTaskSnapshot {
            task_id,
            id,
            status: result_status.or(status),
            state,
            model,
            videos: provider_assets_from_generated_media(videos),
            error: error.map(provider_error_from_claw_router),
        },
    )
    .map_err(sdk_error_from_normalization)
}

fn provider_assets_from_generated_media(
    assets: Vec<ProviderGeneratedMedia>,
) -> Vec<ProviderGeneratedVideoAsset> {
    assets
        .into_iter()
        .map(|asset| ProviderGeneratedVideoAsset {
            id: asset.id,
            uri: asset.uri,
            url: asset.url,
            mime_type: asset.mime_type,
            width: asset.width,
            height: asset.height,
            duration_seconds: asset.duration.map(|value| value.round() as i32),
        })
        .collect()
}

fn provider_media_from_volcengine_content_parts(
    content: Vec<VolcengineContentPart>,
) -> Vec<ProviderGeneratedMedia> {
    content
        .into_iter()
        .filter_map(|part| {
            part.video_url.map(|video_url| ProviderGeneratedMedia {
                id: part.file_id,
                uri: None,
                url: Some(video_url),
                mime_type: Some("video/mp4".to_string()),
                ..ProviderGeneratedMedia::default()
            })
        })
        .collect()
}

fn provider_assets_from_vidu_creations(
    creations: Vec<ViduCreation>,
) -> Vec<ProviderGeneratedVideoAsset> {
    creations
        .into_iter()
        .filter_map(|creation| {
            vidu_creation_video_url(&creation).map(|url| ProviderGeneratedVideoAsset {
                id: creation.id,
                uri: creation.uri,
                url: Some(url),
                mime_type: Some("video/mp4".to_string()),
                width: creation.width,
                height: creation.height,
                duration_seconds: creation.duration.map(|value| value.round() as i32),
            })
        })
        .collect()
}

fn vidu_creation_video_url(creation: &ViduCreation) -> Option<String> {
    if let Some(video_url) = normalized_optional_text(creation.video_url.as_deref()) {
        return Some(video_url);
    }

    let creation_type = creation
        .r#type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase);
    if creation_type
        .as_deref()
        .is_some_and(|value| value.contains("image") || value.contains("audio"))
    {
        return None;
    }

    let url = normalized_optional_text(creation.url.as_deref())?;
    if creation_type
        .as_deref()
        .is_some_and(|value| value.contains("video"))
        || looks_like_video_url(&url)
        || (creation_type.is_none()
            && creation.image_url.is_none()
            && creation.audio_url.is_none()
            && creation.cover_url.is_none())
    {
        Some(url)
    } else {
        None
    }
}

fn normalized_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn looks_like_video_url(url: &str) -> bool {
    let path = url
        .split(['?', '#'])
        .next()
        .unwrap_or(url)
        .to_ascii_lowercase();
    [".mp4", ".webm", ".mov", ".m4v", ".mkv"]
        .iter()
        .any(|extension| path.ends_with(extension))
}

fn provider_error_from_claw_router(error: ProviderTaskError) -> ProviderTaskErrorSnapshot {
    ProviderTaskErrorSnapshot {
        code: error.code,
        message: error.message,
        error_type: error.r#type,
    }
}

pub(crate) fn sdk_error_from_normalization(error: &'static str) -> SdkworkError {
    SdkworkError::Serialization(serde_json::Error::io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        error,
    )))
}

#[cfg(test)]
mod tests {
    use clawrouter_open_sdk::{
        OpenAiVideo, ProviderGeneratedMedia, ProviderTaskResult, ViduCreation,
        ViduTaskCreationsResponse, VolcengineContentGenerationTask, VolcengineContentPart,
    };
    use sdkwork_video_core::{
        plan_video_generation_provider_dispatch, VideoGenerationCreateCommand,
        VideoGenerationRuntimeStatus,
    };

    use super::{
        normalize_openai_video, normalize_vidu_task_creations_response,
        normalize_volcengine_generation_task,
    };

    #[test]
    fn normalizes_vidu_task_creations_without_importing_non_video_creation_urls() {
        let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
            prompt: "Vidu mixed creation payload".to_string(),
            negative_prompt: None,
            scene: "vidu_mixed_creation".to_string(),
            provider_code: Some("vidu".to_string()),
            operation: Some("text_to_video".to_string()),
            model: Some("vidu2.0".to_string()),
            resolution: Some("1080p".to_string()),
            aspect_ratio: Some("16:9".to_string()),
            duration_seconds: Some(8),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: None,
            webhook_url: None,
            idempotency_key: None,
        })
        .expect("vidu dispatch plan should build");
        let normalized = normalize_vidu_task_creations_response(
            &plan,
            ViduTaskCreationsResponse {
                task_id: Some("task-vidu-mixed".to_string()),
                state: Some("success".to_string()),
                model: Some("vidu2.0".to_string()),
                creations: Some(vec![ViduCreation {
                    id: Some("cover-0".to_string()),
                    image_url: Some("https://provider.example.com/vidu-cover.png".to_string()),
                    r#type: Some("image".to_string()),
                    url: Some("https://provider.example.com/vidu-cover.png".to_string()),
                    ..ViduCreation::default()
                }]),
                ..ViduTaskCreationsResponse::default()
            },
        )
        .expect("vidu task creations should normalize");

        assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Succeeded);
        assert!(normalized.provider_terminal);
        assert!(!normalized.ready_for_drive_import);
        assert!(normalized.outputs.is_empty());
    }

    #[test]
    fn normalizes_openai_compatible_signed_video_url_without_forcing_mp4_mime_type() {
        let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
            prompt: "OpenAI-compatible signed video".to_string(),
            negative_prompt: None,
            scene: "openai_signed_video".to_string(),
            provider_code: Some("openai".to_string()),
            operation: None,
            model: Some("sora-video".to_string()),
            resolution: Some("1280x720".to_string()),
            aspect_ratio: None,
            duration_seconds: Some(6),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: None,
            webhook_url: None,
            idempotency_key: None,
        })
        .expect("openai-compatible dispatch plan should build");
        let normalized = normalize_openai_video(
            &plan,
            OpenAiVideo {
                id: "video-openai-webm".to_string(),
                object: "video".to_string(),
                status: "succeeded".to_string(),
                url: Some("https://provider.example.com/openai-output.webm?sig=abc".to_string()),
                model: Some("sora-video".to_string()),
                seconds: Some(6),
                ..OpenAiVideo::default()
            },
        )
        .expect("openai-compatible video should normalize");

        assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Importing);
        assert_eq!(normalized.outputs.len(), 1);
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
    fn normalizes_volcengine_result_videos_from_generated_task_result_payload() {
        let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
            prompt: "Volcengine result payload".to_string(),
            negative_prompt: None,
            scene: "volcengine_result".to_string(),
            provider_code: Some("volcengine".to_string()),
            operation: None,
            model: Some("doubao-seedance".to_string()),
            resolution: None,
            aspect_ratio: None,
            duration_seconds: Some(6),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: None,
            webhook_url: None,
            idempotency_key: None,
        })
        .expect("volcengine dispatch plan should build");
        let normalized = normalize_volcengine_generation_task(
            &plan,
            VolcengineContentGenerationTask {
                id: Some("volc-result-id".to_string()),
                task_id: Some("task-volc-result".to_string()),
                model: Some("doubao-seedance".to_string()),
                result: Some(ProviderTaskResult {
                    status: Some("succeeded".to_string()),
                    videos: Some(vec![ProviderGeneratedMedia {
                        id: Some("asset-result-0".to_string()),
                        uri: Some(
                            "provider://volcengine/tasks/task-volc-result/videos/0".to_string(),
                        ),
                        url: Some("https://provider.example.com/volc-result-0.mp4".to_string()),
                        mime_type: Some("video/mp4".to_string()),
                        width: Some(1280),
                        height: Some(720),
                        duration: Some(6.0),
                        ..ProviderGeneratedMedia::default()
                    }]),
                    ..ProviderTaskResult::default()
                }),
                ..VolcengineContentGenerationTask::default()
            },
        )
        .expect("volcengine generated task should normalize");

        assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Importing);
        assert!(normalized.provider_terminal);
        assert!(normalized.ready_for_drive_import);
        assert_eq!(
            normalized.provider_task_id.as_deref(),
            Some("task-volc-result")
        );
        assert_eq!(normalized.outputs.len(), 1);
        assert_eq!(
            normalized.outputs[0].provider_asset_id.as_deref(),
            Some("asset-result-0")
        );
        assert_eq!(normalized.outputs[0].duration_seconds, Some(6));
    }

    #[test]
    fn normalizes_volcengine_result_content_video_url_as_generated_output() {
        let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
            prompt: "Volcengine content payload".to_string(),
            negative_prompt: None,
            scene: "volcengine_content_result".to_string(),
            provider_code: Some("volcengine".to_string()),
            operation: None,
            model: Some("doubao-seedance".to_string()),
            resolution: None,
            aspect_ratio: None,
            duration_seconds: Some(6),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: None,
            webhook_url: None,
            idempotency_key: None,
        })
        .expect("volcengine dispatch plan should build");
        let normalized = normalize_volcengine_generation_task(
            &plan,
            VolcengineContentGenerationTask {
                id: Some("volc-content-id".to_string()),
                task_id: Some("task-volc-content".to_string()),
                model: Some("doubao-seedance".to_string()),
                result: Some(ProviderTaskResult {
                    status: Some("succeeded".to_string()),
                    content: Some(vec![VolcengineContentPart {
                        file_id: Some("file-content-0".to_string()),
                        video_url: Some(
                            "https://provider.example.com/volc-content-0.mp4".to_string(),
                        ),
                        r#type: "video_url".to_string(),
                        ..VolcengineContentPart::default()
                    }]),
                    ..ProviderTaskResult::default()
                }),
                ..VolcengineContentGenerationTask::default()
            },
        )
        .expect("volcengine generated task should normalize");

        assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Importing);
        assert!(normalized.ready_for_drive_import);
        assert_eq!(normalized.outputs.len(), 1);
        assert_eq!(
            normalized.outputs[0].provider_asset_id.as_deref(),
            Some("file-content-0")
        );
        assert_eq!(
            normalized.outputs[0].provider_url.as_deref(),
            Some("https://provider.example.com/volc-content-0.mp4")
        );
    }

    #[test]
    fn normalizes_volcengine_top_level_content_video_url_as_generated_output() {
        let plan = plan_video_generation_provider_dispatch(&VideoGenerationCreateCommand {
            prompt: "Volcengine top-level content payload".to_string(),
            negative_prompt: None,
            scene: "volcengine_top_level_content".to_string(),
            provider_code: Some("volcengine".to_string()),
            operation: None,
            model: Some("doubao-seedance".to_string()),
            resolution: None,
            aspect_ratio: None,
            duration_seconds: Some(6),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: None,
            webhook_url: None,
            idempotency_key: None,
        })
        .expect("volcengine dispatch plan should build");
        let normalized = normalize_volcengine_generation_task(
            &plan,
            VolcengineContentGenerationTask {
                id: Some("volc-top-content-id".to_string()),
                task_id: Some("task-volc-top-content".to_string()),
                model: Some("doubao-seedance".to_string()),
                status: Some("succeeded".to_string()),
                content: Some(vec![VolcengineContentPart {
                    file_id: Some("file-top-content-0".to_string()),
                    video_url: Some(
                        "https://provider.example.com/volc-top-content-0.mp4".to_string(),
                    ),
                    r#type: "video_url".to_string(),
                    ..VolcengineContentPart::default()
                }]),
                ..VolcengineContentGenerationTask::default()
            },
        )
        .expect("volcengine generated task should normalize");

        assert_eq!(normalized.status, VideoGenerationRuntimeStatus::Importing);
        assert!(normalized.ready_for_drive_import);
        assert_eq!(normalized.outputs.len(), 1);
        assert_eq!(
            normalized.outputs[0].provider_asset_id.as_deref(),
            Some("file-top-content-0")
        );
        assert_eq!(
            normalized.outputs[0].provider_url.as_deref(),
            Some("https://provider.example.com/volc-top-content-0.mp4")
        );
    }
}
