use crate::{
    media_format::{i64_to_i32, infer_video_mime_type_from_url, video_file_extension_for_mime},
    models::{GeneratedVideoOutput, NormalizedProviderVideoGenerationResult, ProviderTaskSnapshot},
    status::VideoGenerationRuntimeStatus,
    text::{normalize_provider_code_for_storage, normalized_optional_text, require_trimmed},
};

pub fn normalize_provider_task_video_generation_result(
    provider_code: impl AsRef<str>,
    task: ProviderTaskSnapshot,
) -> Result<NormalizedProviderVideoGenerationResult, &'static str> {
    let provider_code = normalize_provider_code_for_storage(require_trimmed(
        provider_code.as_ref(),
        "video generation provider_code is required",
    )?);
    let provider_status = normalized_optional_text(task.status.as_deref());
    let provider_state = normalized_optional_text(task.state.as_deref());
    let normalized_state = provider_status
        .as_deref()
        .or(provider_state.as_deref())
        .unwrap_or(if task.videos.is_empty() {
            "submitted"
        } else {
            "completed"
        });
    let status = normalize_provider_status(
        normalized_state,
        !task.videos.is_empty(),
        task.error.is_some(),
    );
    let provider_terminal = matches!(
        status,
        VideoGenerationRuntimeStatus::Importing
            | VideoGenerationRuntimeStatus::Succeeded
            | VideoGenerationRuntimeStatus::Failed
            | VideoGenerationRuntimeStatus::Cancelled
            | VideoGenerationRuntimeStatus::Expired
    );
    let ready_for_drive_import = matches!(
        status,
        VideoGenerationRuntimeStatus::Importing | VideoGenerationRuntimeStatus::Succeeded
    ) && !task.videos.is_empty();

    let outputs = task
        .videos
        .into_iter()
        .enumerate()
        .map(|(index, asset)| {
            let mime_type = normalized_optional_text(asset.mime_type.as_deref())
                .or_else(|| infer_video_mime_type_from_url(asset.url.as_deref()))
                .or_else(|| Some("video/mp4".to_string()));

            GeneratedVideoOutput {
                output_index: index as i32,
                provider_asset_id: normalized_optional_text(asset.id.as_deref()),
                provider_uri: normalized_optional_text(asset.uri.as_deref()).or_else(|| {
                    Some(format!(
                        "provider://{}/tasks/{}/videos/{}",
                        provider_code,
                        task.task_id
                            .as_deref()
                            .map(str::trim)
                            .filter(|value| !value.is_empty())
                            .unwrap_or("unknown"),
                        index
                    ))
                }),
                provider_url: normalized_optional_text(asset.url.as_deref()),
                file_name: Some(format!(
                    "generated-{}.{}",
                    index,
                    video_file_extension_for_mime(mime_type.as_deref())
                )),
                mime_type,
                size_bytes: None,
                width: i64_to_i32(asset.width),
                height: i64_to_i32(asset.height),
                duration_seconds: asset.duration_seconds,
            }
        })
        .collect::<Vec<_>>();

    let (error_code, error_message) = match task.error {
        Some(error) => (
            normalized_optional_text(error.code.as_deref())
                .or_else(|| normalized_optional_text(error.error_type.as_deref())),
            normalized_optional_text(error.message.as_deref()),
        ),
        None => (None, None),
    };

    Ok(NormalizedProviderVideoGenerationResult {
        provider_code,
        provider_task_id: normalized_optional_text(task.task_id.as_deref())
            .or_else(|| normalized_optional_text(task.id.as_deref())),
        provider_status,
        provider_state,
        status,
        provider_terminal,
        ready_for_drive_import,
        outputs,
        error_code,
        error_message,
    })
}

fn normalize_provider_status(
    status: &str,
    has_outputs: bool,
    has_error: bool,
) -> VideoGenerationRuntimeStatus {
    let normalized = status.trim().to_ascii_lowercase();
    if has_error {
        return VideoGenerationRuntimeStatus::Failed;
    }
    if matches!(
        normalized.as_str(),
        "succeed" | "succeeded" | "success" | "completed" | "complete" | "done" | "finished"
    ) {
        return if has_outputs {
            VideoGenerationRuntimeStatus::Importing
        } else {
            VideoGenerationRuntimeStatus::Succeeded
        };
    }
    if matches!(
        normalized.as_str(),
        "failed" | "failure" | "error" | "rejected" | "blocked"
    ) {
        return VideoGenerationRuntimeStatus::Failed;
    }
    if matches!(normalized.as_str(), "cancelled" | "canceled") {
        return VideoGenerationRuntimeStatus::Cancelled;
    }
    if matches!(normalized.as_str(), "expired" | "timeout" | "timed_out") {
        return VideoGenerationRuntimeStatus::Expired;
    }
    if matches!(
        normalized.as_str(),
        "running" | "processing" | "rendering" | "in_progress"
    ) {
        return VideoGenerationRuntimeStatus::Rendering;
    }
    if matches!(
        normalized.as_str(),
        "created" | "queued" | "queueing" | "pending" | "submitted"
    ) {
        return VideoGenerationRuntimeStatus::Submitted;
    }
    VideoGenerationRuntimeStatus::Rendering
}
