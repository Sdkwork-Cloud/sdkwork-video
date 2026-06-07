use sdkwork_video_core::{
    DriveGeneratedVideoImportPlan, NormalizedProviderVideoGenerationResult,
    VideoGenerationCreateCommand, VideoGenerationRuntimeStatus,
};

use crate::{
    models::{
        VideoGenerationOutputPersistenceRow, VideoGenerationPersistencePlan, VideoGenerationRecord,
        VideoGenerationRefreshPlan, VideoGenerationScope,
    },
    planning::{
        plan_video_generation_create_service_flow,
        plan_video_generation_refresh_from_provider_result,
    },
    text::require_trimmed_owned,
};

pub fn plan_video_generation_create_persistence_plan(
    scope: VideoGenerationScope,
    generation_id: impl Into<String>,
    command: VideoGenerationCreateCommand,
    provider_result: Option<NormalizedProviderVideoGenerationResult>,
) -> Result<VideoGenerationPersistencePlan, &'static str> {
    let service_plan =
        plan_video_generation_create_service_flow(scope, generation_id, command, provider_result)?;
    Ok(persistence_plan_from_service_plan(
        &service_plan.record,
        &service_plan.dispatch.normalized_result,
        &service_plan.drive_import_plans,
        true,
    ))
}

pub fn plan_video_generation_refresh_persistence_plan(
    scope: VideoGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    result: NormalizedProviderVideoGenerationResult,
) -> Result<VideoGenerationPersistencePlan, &'static str> {
    let plan = plan_video_generation_refresh_from_provider_result(
        scope,
        generation_id,
        scene,
        model,
        result,
    )?;
    Ok(persistence_plan_from_refresh_plan(&plan))
}

pub fn plan_video_generation_drive_import_completion_persistence_plan(
    generation_id: impl Into<String>,
    provider_code: impl Into<String>,
    provider_task_id: Option<String>,
    provider_status: Option<String>,
    output_count: i32,
) -> Result<VideoGenerationPersistencePlan, &'static str> {
    let generation_id =
        require_trimmed_owned(generation_id.into(), "video generation id is required")?;
    let provider_code = require_trimmed_owned(
        provider_code.into(),
        "video generation provider_code is required",
    )?;
    if output_count <= 0 {
        return Err("video generation drive completion output_count must be greater than 0");
    }

    Ok(VideoGenerationPersistencePlan {
        generation_id,
        runtime_status: VideoGenerationRuntimeStatus::Succeeded,
        drive_sync_status: VideoGenerationRuntimeStatus::Succeeded
            .as_drive_sync_status()
            .to_string(),
        provider_code,
        provider_task_id,
        provider_status,
        output_rows: vec![],
        repository_methods: vec![
            "mark_drive_imported".to_string(),
            "mark_generation_succeeded".to_string(),
            "enqueue_notification".to_string(),
        ],
    })
}

fn persistence_plan_from_service_plan(
    record: &VideoGenerationRecord,
    normalized_result: &Option<NormalizedProviderVideoGenerationResult>,
    drive_import_plans: &[DriveGeneratedVideoImportPlan],
    include_create_generation: bool,
) -> VideoGenerationPersistencePlan {
    let mut repository_methods = Vec::new();
    if include_create_generation {
        repository_methods.push("create_generation".to_string());
    }
    repository_methods.push("mark_provider_submitted".to_string());
    if record.provider_task_id.is_some() {
        repository_methods.push("upsert_provider_task".to_string());
    }
    if !drive_import_plans.is_empty() {
        repository_methods.push("upsert_generation_outputs".to_string());
        repository_methods.push("mark_drive_importing".to_string());
    }
    if record.status == VideoGenerationRuntimeStatus::Failed {
        repository_methods.push("mark_generation_failed".to_string());
    }
    if record.status == VideoGenerationRuntimeStatus::Succeeded {
        repository_methods.push("mark_generation_succeeded".to_string());
    }
    repository_methods.push("enqueue_notification".to_string());

    VideoGenerationPersistencePlan {
        generation_id: record.generation_id.clone(),
        runtime_status: record.status,
        drive_sync_status: record.status.as_drive_sync_status().to_string(),
        provider_code: record.provider_code.clone(),
        provider_task_id: record.provider_task_id.clone(),
        provider_status: record.provider_status.clone().or_else(|| {
            normalized_result
                .as_ref()
                .and_then(|result| result.provider_status.clone())
        }),
        output_rows: drive_import_plans
            .iter()
            .map(output_persistence_row_from_drive_plan)
            .collect(),
        repository_methods,
    }
}

fn persistence_plan_from_refresh_plan(
    plan: &VideoGenerationRefreshPlan,
) -> VideoGenerationPersistencePlan {
    let mut repository_methods = vec!["mark_provider_submitted".to_string()];
    if !plan.drive_import_plans.is_empty() {
        repository_methods.push("upsert_generation_outputs".to_string());
        repository_methods.push("mark_drive_importing".to_string());
    }
    if plan.status == VideoGenerationRuntimeStatus::Failed {
        repository_methods.push("mark_generation_failed".to_string());
    }
    if plan.status == VideoGenerationRuntimeStatus::Succeeded {
        repository_methods.push("mark_generation_succeeded".to_string());
    }
    for _ in &plan.outbox_events {
        repository_methods.push("enqueue_notification".to_string());
    }

    VideoGenerationPersistencePlan {
        generation_id: plan.generation_id.clone(),
        runtime_status: plan.status,
        drive_sync_status: plan.status.as_drive_sync_status().to_string(),
        provider_code: plan.provider_code.clone(),
        provider_task_id: plan.provider_task_id.clone(),
        provider_status: plan.provider_status.clone(),
        output_rows: plan
            .drive_import_plans
            .iter()
            .map(output_persistence_row_from_drive_plan)
            .collect(),
        repository_methods,
    }
}

fn output_persistence_row_from_drive_plan(
    plan: &DriveGeneratedVideoImportPlan,
) -> VideoGenerationOutputPersistenceRow {
    VideoGenerationOutputPersistenceRow {
        output_index: plan.output_index,
        media_kind: plan.media_resource.kind.clone(),
        scene: plan.scene.clone(),
        provider_code: plan.provider_code.clone(),
        provider_asset_id: plan.provider_asset_id.clone(),
        provider_uri: plan.provider_uri.clone(),
        provider_url: plan.provider_url.clone(),
        drive_space_type: plan.drive_space_type.clone(),
        drive_space_id: plan.drive_space_id.clone(),
        drive_parent_node_id: plan.drive_parent_node_id.clone(),
        drive_node_id: plan.drive_node_id.clone(),
        drive_uri: plan.drive_uri.clone(),
        resource_snapshot_id: plan.media_resource.id.clone(),
        file_name: plan.media_resource.file_name.clone(),
        mime_type: plan.media_resource.mime_type.clone(),
        size_bytes: plan.media_resource.size_bytes.clone(),
        width: plan.media_resource.width,
        height: plan.media_resource.height,
        duration_seconds: plan.media_resource.duration_seconds,
        sync_status: "pending".to_string(),
    }
}
