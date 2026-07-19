use sdkwork_video_core::{
    plan_drive_import_for_generated_video_outputs, plan_video_generation_provider_dispatch,
    DriveGeneratedVideoContext, GeneratedVideoOutput, NormalizedProviderVideoGenerationResult,
    VideoGenerationCreateCommand, VideoGenerationRuntimeStatus, VideoProviderDispatchPlan,
};

use crate::{
    models::{
        VideoGenerationOutboxEvent, VideoGenerationProviderSubmission, VideoGenerationRecord,
        VideoGenerationRefreshPlan, VideoGenerationRuntimeStep, VideoGenerationScope,
        VideoGenerationServicePlan, VideoGenerationWebhookEnvelope,
    },
    text::require_trimmed_owned,
};

pub fn plan_video_generation_create_service_flow(
    scope: VideoGenerationScope,
    generation_id: impl Into<String>,
    command: VideoGenerationCreateCommand,
    provider_result: Option<NormalizedProviderVideoGenerationResult>,
) -> Result<VideoGenerationServicePlan, &'static str> {
    let generation_id =
        require_trimmed_owned(generation_id.into(), "video generation id is required")?;
    let dispatch_plan = plan_video_generation_provider_dispatch(&command)?;
    let outputs = provider_result
        .as_ref()
        .filter(|result| result.ready_for_drive_import)
        .map(|result| result.outputs.clone())
        .unwrap_or_default();
    let drive_import_plans = if outputs.is_empty() {
        Vec::new()
    } else {
        plan_drive_imports(
            &scope,
            &generation_id,
            &dispatch_plan,
            &command.scene,
            outputs,
        )?
    };
    let status = provider_result
        .as_ref()
        .map(|result| result.status)
        .unwrap_or(VideoGenerationRuntimeStatus::Dispatching);
    let provider_task_id = provider_result
        .as_ref()
        .and_then(|result| result.provider_task_id.clone());
    let provider_status = provider_result
        .as_ref()
        .and_then(|result| result.provider_status.clone());
    let drive_space_id = drive_import_plans
        .first()
        .map(|plan| plan.drive_space_id.clone());

    Ok(VideoGenerationServicePlan {
        record: VideoGenerationRecord {
            generation_id: generation_id.clone(),
            status,
            scene: command.scene,
            provider_code: dispatch_plan.provider_code.clone(),
            provider_task_id,
            provider_status,
            drive_space_id,
        },
        dispatch: VideoGenerationProviderSubmission {
            generation_id: generation_id.clone(),
            dispatch_plan,
            normalized_result: provider_result,
        },
        drive_import_plans,
        outbox_events: vec![VideoGenerationOutboxEvent {
            aggregate_type: "video_generation".to_string(),
            aggregate_id: generation_id,
            event_type: "video.generation.created".to_string(),
        }],
    })
}

pub fn plan_video_generation_create_runtime_steps(
    scope: VideoGenerationScope,
    generation_id: impl Into<String>,
    command: VideoGenerationCreateCommand,
) -> Result<Vec<VideoGenerationRuntimeStep>, &'static str> {
    let plan = plan_video_generation_create_service_flow(scope, generation_id, command, None)?;
    let dispatch_plan = &plan.dispatch.dispatch_plan;
    let mut steps = vec![
        VideoGenerationRuntimeStep::CreateGenerationRecord,
        VideoGenerationRuntimeStep::DispatchProviderGeneration {
            provider_id: dispatch_plan.provider_id.clone(),
            provider_code: dispatch_plan.provider_code.clone(),
        },
        VideoGenerationRuntimeStep::PersistProviderSubmission,
    ];

    if dispatch_plan.callback_url.is_some() {
        steps.push(VideoGenerationRuntimeStep::AwaitProviderWebhook);
    }
    steps.push(VideoGenerationRuntimeStep::ScheduleProviderPolling);
    steps.push(VideoGenerationRuntimeStep::PersistOutboxEvent {
        event_type: "video.generation.created".to_string(),
    });

    Ok(steps)
}

pub fn plan_video_generation_refresh_from_provider_result(
    scope: VideoGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    result: NormalizedProviderVideoGenerationResult,
) -> Result<VideoGenerationRefreshPlan, &'static str> {
    let generation_id =
        require_trimmed_owned(generation_id.into(), "video generation id is required")?;
    let scene = require_trimmed_owned(scene.into(), "video generation scene is required")?;
    let provider_code = require_trimmed_owned(
        result.provider_code.clone(),
        "video generation provider_code is required",
    )?;
    let drive_import_plans = if result.ready_for_drive_import && !result.outputs.is_empty() {
        plan_drive_import_for_generated_video_outputs(
            DriveGeneratedVideoContext {
                tenant_id: scope.tenant_id,
                organization_id: scope.organization_id,
                generation_id: generation_id.clone(),
                provider_code: provider_code.clone(),
                model,
                scene,
                actor: scope.actor,
            },
            result.outputs.clone(),
        )?
    } else {
        Vec::new()
    };
    let event_type = if result.status == VideoGenerationRuntimeStatus::Failed {
        "video.generation.failed"
    } else if result.ready_for_drive_import {
        "video.generation.outputs_ready"
    } else if result.status == VideoGenerationRuntimeStatus::Succeeded {
        "video.generation.succeeded"
    } else {
        "video.generation.refreshed"
    };

    Ok(VideoGenerationRefreshPlan {
        generation_id: generation_id.clone(),
        status: result.status,
        provider_code,
        provider_task_id: result.provider_task_id,
        provider_status: result.provider_status,
        drive_import_plans,
        outbox_events: vec![VideoGenerationOutboxEvent {
            aggregate_type: "video_generation".to_string(),
            aggregate_id: generation_id,
            event_type: event_type.to_string(),
        }],
    })
}

pub fn plan_video_generation_refresh_runtime_steps(
    scope: VideoGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    result: NormalizedProviderVideoGenerationResult,
) -> Result<Vec<VideoGenerationRuntimeStep>, &'static str> {
    let plan = plan_video_generation_refresh_from_provider_result(
        scope,
        generation_id,
        scene,
        model,
        result,
    )?;
    let mut steps = vec![VideoGenerationRuntimeStep::PersistProviderSubmission];
    let output_count = i32::try_from(plan.drive_import_plans.len())
        .map_err(|_| "video generation output_count exceeds supported range")?;
    if output_count > 0 {
        steps.push(VideoGenerationRuntimeStep::PersistDriveImportPlan { output_count });
        steps.push(VideoGenerationRuntimeStep::PrepareDriveUpload { output_count });
    }
    for event in plan.outbox_events {
        steps.push(VideoGenerationRuntimeStep::PersistOutboxEvent {
            event_type: event.event_type,
        });
    }
    Ok(steps)
}

pub fn plan_video_generation_refresh_from_webhook(
    scope: VideoGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    webhook: VideoGenerationWebhookEnvelope,
) -> Result<VideoGenerationRefreshPlan, &'static str> {
    if webhook.payload_hash.trim().is_empty() {
        return Err("video generation webhook payload_hash is required");
    }
    if webhook.event_type.trim().is_empty() {
        return Err("video generation webhook event_type is required");
    }
    if webhook.provider_code.trim() != webhook.normalized_result.provider_code.trim() {
        return Err("video generation webhook provider_code does not match normalized result");
    }
    if webhook.provider_task_id != webhook.normalized_result.provider_task_id {
        return Err("video generation webhook provider_task_id does not match normalized result");
    }

    plan_video_generation_refresh_from_provider_result(
        scope,
        generation_id,
        scene,
        model,
        webhook.normalized_result,
    )
}

pub fn plan_video_generation_drive_import_completion_runtime_steps(
    generation_id: impl Into<String>,
    output_count: i32,
) -> Result<Vec<VideoGenerationRuntimeStep>, &'static str> {
    require_trimmed_owned(generation_id.into(), "video generation id is required")?;
    if output_count <= 0 {
        return Err("video generation drive completion output_count must be greater than 0");
    }

    Ok(vec![
        VideoGenerationRuntimeStep::MarkDriveImported { output_count },
        VideoGenerationRuntimeStep::MarkGenerationSucceeded,
        VideoGenerationRuntimeStep::PersistOutboxEvent {
            event_type: "video.generation.succeeded".to_string(),
        },
    ])
}

fn plan_drive_imports(
    scope: &VideoGenerationScope,
    generation_id: &str,
    dispatch_plan: &VideoProviderDispatchPlan,
    scene: &str,
    outputs: Vec<GeneratedVideoOutput>,
) -> Result<Vec<sdkwork_video_core::DriveGeneratedVideoImportPlan>, &'static str> {
    plan_drive_import_for_generated_video_outputs(
        DriveGeneratedVideoContext {
            tenant_id: scope.tenant_id.clone(),
            organization_id: scope.organization_id.clone(),
            generation_id: generation_id.to_string(),
            provider_code: dispatch_plan.provider_code.clone(),
            model: dispatch_plan.model.clone(),
            scene: scene.to_string(),
            actor: scope.actor.clone(),
        },
        outputs,
    )
}
