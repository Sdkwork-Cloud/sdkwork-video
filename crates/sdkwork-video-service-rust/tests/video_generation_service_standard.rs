use sdkwork_video_core::{
    normalize_provider_task_video_generation_result, ProviderGeneratedVideoAsset,
    ProviderTaskErrorSnapshot, ProviderTaskSnapshot, VideoGenerationActor,
    VideoGenerationCreateCommand, VideoGenerationRuntimeStatus,
};
use sdkwork_video_service::{
    plan_video_generation_create_persistence_plan, plan_video_generation_create_runtime_steps,
    plan_video_generation_create_service_flow,
    plan_video_generation_drive_import_completion_persistence_plan,
    plan_video_generation_drive_import_completion_runtime_steps,
    plan_video_generation_refresh_from_provider_result, plan_video_generation_refresh_from_webhook,
    plan_video_generation_refresh_persistence_plan, plan_video_generation_refresh_runtime_steps,
    video_generation_repository_contract_methods, VideoGenerationRuntimeStep, VideoGenerationScope,
    VideoGenerationWebhookEnvelope,
};

#[test]
fn keeps_lib_rs_as_public_module_boundary() {
    let lib_rs = include_str!("../src/lib.rs");

    assert!(
        lib_rs.lines().count() <= 80,
        "sdkwork-video-service-rust src/lib.rs must stay a small module assembly boundary"
    );
    for forbidden in ["pub struct ", "pub enum ", "pub fn ", "impl "] {
        assert!(
            !lib_rs.contains(forbidden),
            "src/lib.rs must not contain authored service logic marker {forbidden}"
        );
    }
    assert!(lib_rs.contains("pub mod "));
    assert!(lib_rs.contains("pub use "));
}

#[test]
fn plans_create_flow_with_provider_result_drive_import_and_outbox() {
    let result = normalize_provider_task_video_generation_result(
        "kling",
        ProviderTaskSnapshot {
            task_id: Some("task-001".to_string()),
            id: None,
            status: Some("succeeded".to_string()),
            state: Some("completed".to_string()),
            model: Some("kling-v2".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-001".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-001.mp4".to_string()),
                mime_type: Some("video/mp4".to_string()),
                width: Some(1280),
                height: Some(720),
                duration_seconds: Some(5),
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_video_generation_create_service_flow(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-001",
        VideoGenerationCreateCommand {
            prompt: "Product reveal".to_string(),
            negative_prompt: None,
            scene: "product_reveal".to_string(),
            provider_code: Some("kling".to_string()),
            operation: None,
            model: Some("kling-v2".to_string()),
            resolution: Some("720p".to_string()),
            aspect_ratio: Some("16:9".to_string()),
            duration_seconds: Some(5),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: Some("pro".to_string()),
            webhook_url: Some("https://app.example.com/hooks/video".to_string()),
            idempotency_key: Some("idem-001".to_string()),
        },
        Some(result),
    )
    .expect("service flow should plan");

    assert_eq!(plan.record.generation_id, "video-generation-001");
    assert_eq!(plan.record.status, VideoGenerationRuntimeStatus::Importing);
    assert_eq!(plan.record.scene, "product_reveal");
    assert_eq!(plan.record.provider_code, "kling");
    assert_eq!(plan.record.provider_task_id.as_deref(), Some("task-001"));
    assert!(plan.dispatch.dispatch_plan.provider_id.is_empty());
    assert_eq!(plan.drive_import_plans.len(), 1);
    assert_eq!(plan.drive_import_plans[0].scene, "product_reveal");
    assert_eq!(plan.drive_import_plans[0].drive_space_type, "ai_generated");
    assert_eq!(
        plan.drive_import_plans[0].drive_upload_profile_code,
        "video"
    );
    assert_eq!(
        plan.drive_import_plans[0].drive_owner_subject_id,
        "user-001",
    );
    assert_eq!(plan.outbox_events[0].event_type, "video.generation.created");
}

#[test]
fn plans_refresh_from_polling_result_with_drive_import_outputs_ready_event() {
    let result = normalize_provider_task_video_generation_result(
        "vidu",
        ProviderTaskSnapshot {
            task_id: Some("task-002".to_string()),
            id: None,
            status: Some("completed".to_string()),
            state: None,
            model: Some("vidu2.0".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-002".to_string()),
                uri: Some("provider://vidu/tasks/task-002/videos/0".to_string()),
                url: Some("https://provider.example.com/asset-002.mp4".to_string()),
                mime_type: Some("video/mp4".to_string()),
                width: Some(1920),
                height: Some(1080),
                duration_seconds: Some(8),
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_video_generation_refresh_from_provider_result(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: VideoGenerationActor::Anonymous {
                anonymous_id: "anon-001".to_string(),
            },
        },
        "video-generation-002",
        "anonymous_tryout",
        Some("vidu2.0".to_string()),
        result,
    )
    .expect("refresh plan should build");

    assert_eq!(plan.status, VideoGenerationRuntimeStatus::Importing);
    assert_eq!(plan.provider_task_id.as_deref(), Some("task-002"));
    assert_eq!(plan.drive_import_plans.len(), 1);
    assert_eq!(
        plan.drive_import_plans[0].drive_owner_subject_id,
        "app:sdkwork-video:anonymous",
    );
    assert_eq!(plan.drive_import_plans[0].drive_actor_type, "anonymous");
    assert_eq!(plan.drive_import_plans[0].drive_actor_id, "anon-001");
    assert_eq!(
        plan.outbox_events[0].event_type,
        "video.generation.outputs_ready",
    );
}

#[test]
fn validates_webhook_consistency_before_planning_refresh() {
    let result = normalize_provider_task_video_generation_result(
        "volcengine",
        ProviderTaskSnapshot {
            task_id: Some("task-003".to_string()),
            id: None,
            status: Some("succeeded".to_string()),
            state: None,
            model: Some("doubao-seedance".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-003".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-003.mp4".to_string()),
                mime_type: Some("video/mp4".to_string()),
                width: None,
                height: None,
                duration_seconds: Some(6),
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_video_generation_refresh_from_webhook(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-003",
        "webhook_scene",
        Some("doubao-seedance".to_string()),
        VideoGenerationWebhookEnvelope {
            provider_code: "volcengine".to_string(),
            provider_task_id: Some("task-003".to_string()),
            external_event_id: Some("evt-003".to_string()),
            event_type: "generation.succeeded".to_string(),
            payload_hash: "hash-003".to_string(),
            normalized_result: result,
        },
    )
    .expect("webhook refresh plan should build");

    assert_eq!(plan.generation_id, "video-generation-003");
    assert_eq!(plan.drive_import_plans.len(), 1);
    assert_eq!(
        plan.outbox_events[0].event_type,
        "video.generation.outputs_ready",
    );
}

#[test]
fn declares_repository_methods_needed_for_complete_runtime_consistency() {
    let methods = video_generation_repository_contract_methods();

    for expected in [
        "create_generation",
        "mark_provider_submitted",
        "upsert_provider_task",
        "record_provider_webhook_event",
        "upsert_generation_outputs",
        "mark_drive_importing",
        "mark_drive_imported",
        "mark_generation_succeeded",
        "mark_generation_failed",
        "enqueue_notification",
        "find_due_provider_tasks",
        "find_pending_drive_imports",
    ] {
        assert!(
            methods.contains(&expected),
            "missing repository method {expected}"
        );
    }
}

#[test]
fn plans_create_persistence_bindings_with_drive_sync_state() {
    let result = normalize_provider_task_video_generation_result(
        "kling",
        ProviderTaskSnapshot {
            task_id: Some("task-persist-001".to_string()),
            id: None,
            status: Some("succeeded".to_string()),
            state: Some("completed".to_string()),
            model: Some("kling-v2".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-persist-001".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-persist-001.mp4".to_string()),
                mime_type: Some("video/mp4".to_string()),
                width: Some(1280),
                height: Some(720),
                duration_seconds: Some(5),
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_video_generation_create_persistence_plan(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-persist-001",
        VideoGenerationCreateCommand {
            prompt: "Product reveal".to_string(),
            negative_prompt: None,
            scene: "product_reveal".to_string(),
            provider_code: Some("kling".to_string()),
            operation: None,
            model: Some("kling-v2".to_string()),
            resolution: Some("720p".to_string()),
            aspect_ratio: Some("16:9".to_string()),
            duration_seconds: Some(5),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: Some("pro".to_string()),
            webhook_url: Some("https://app.example.com/hooks/video".to_string()),
            idempotency_key: Some("idem-persist-001".to_string()),
        },
        Some(result),
    )
    .expect("persistence plan should build");

    assert_eq!(plan.generation_id, "video-generation-persist-001");
    assert_eq!(plan.runtime_status, VideoGenerationRuntimeStatus::Importing);
    assert_eq!(plan.drive_sync_status, "importing");
    assert_eq!(plan.repository_methods[0], "create_generation");
    assert!(plan
        .repository_methods
        .contains(&"upsert_generation_outputs".to_string()));
    assert!(plan
        .repository_methods
        .contains(&"mark_drive_importing".to_string()));
    assert!(plan
        .repository_methods
        .contains(&"mark_provider_submitted".to_string()));
    assert_eq!(plan.output_rows.len(), 1);
    assert_eq!(plan.output_rows[0].sync_status, "pending");
    assert_eq!(plan.output_rows[0].scene, "product_reveal");
    assert_eq!(plan.output_rows[0].drive_space_type, "ai_generated");
    assert_eq!(plan.output_rows[0].media_kind, "video");
    assert_eq!(plan.output_rows[0].duration_seconds, Some(5));
}

#[test]
fn plans_refresh_persistence_bindings_for_failed_and_ready_outputs() {
    let failure = normalize_provider_task_video_generation_result(
        "vidu",
        ProviderTaskSnapshot {
            task_id: Some("task-persist-failed".to_string()),
            id: None,
            status: Some("failed".to_string()),
            state: None,
            model: None,
            videos: vec![],
            error: Some(ProviderTaskErrorSnapshot {
                code: Some("provider_failed".to_string()),
                message: Some("provider failed".to_string()),
                error_type: None,
            }),
        },
    )
    .expect("failure result should normalize");

    let failed_plan = plan_video_generation_refresh_persistence_plan(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: VideoGenerationActor::Anonymous {
                anonymous_id: "anon-persist".to_string(),
            },
        },
        "video-generation-persist-failed",
        "anonymous_tryout",
        None,
        failure,
    )
    .expect("failed persistence plan should build");

    assert_eq!(failed_plan.drive_sync_status, "failed");
    assert_eq!(failed_plan.provider_code, "vidu");
    assert!(failed_plan
        .repository_methods
        .contains(&"mark_generation_failed".to_string()));
    assert!(failed_plan.output_rows.is_empty());

    let ready = normalize_provider_task_video_generation_result(
        "vidu",
        ProviderTaskSnapshot {
            task_id: Some("task-persist-ready".to_string()),
            id: None,
            status: Some("completed".to_string()),
            state: None,
            model: Some("vidu2.0".to_string()),
            videos: vec![ProviderGeneratedVideoAsset {
                id: Some("asset-persist-ready".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-persist-ready.mp4".to_string()),
                mime_type: Some("video/mp4".to_string()),
                width: None,
                height: None,
                duration_seconds: Some(8),
            }],
            error: None,
        },
    )
    .expect("ready result should normalize");

    let ready_plan = plan_video_generation_refresh_persistence_plan(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: VideoGenerationActor::Anonymous {
                anonymous_id: "anon-persist".to_string(),
            },
        },
        "video-generation-persist-ready",
        "anonymous_tryout",
        Some("vidu2.0".to_string()),
        ready,
    )
    .expect("ready persistence plan should build");

    assert_eq!(ready_plan.drive_sync_status, "importing");
    assert_eq!(ready_plan.provider_code, "vidu");
    assert!(ready_plan
        .repository_methods
        .contains(&"upsert_generation_outputs".to_string()));
    assert!(ready_plan
        .repository_methods
        .contains(&"mark_drive_importing".to_string()));
    assert_eq!(ready_plan.output_rows.len(), 1);
    assert_eq!(ready_plan.output_rows[0].sync_status, "pending");
    assert_eq!(ready_plan.output_rows[0].drive_space_type, "ai_generated");
}

#[test]
fn plans_refresh_success_without_outputs_as_generation_succeeded() {
    let succeeded = normalize_provider_task_video_generation_result(
        "kling",
        ProviderTaskSnapshot {
            task_id: Some("task-succeeded-empty".to_string()),
            id: None,
            status: Some("completed".to_string()),
            state: None,
            model: Some("kling-v2".to_string()),
            videos: vec![],
            error: None,
        },
    )
    .expect("succeeded provider result should normalize");

    let refresh_plan = plan_video_generation_refresh_from_provider_result(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: VideoGenerationActor::User {
                user_id: "user-success".to_string(),
            },
        },
        "video-generation-succeeded-empty",
        "success_scene",
        Some("kling-v2".to_string()),
        succeeded.clone(),
    )
    .expect("success refresh plan should build");

    assert_eq!(refresh_plan.status, VideoGenerationRuntimeStatus::Succeeded);
    assert!(refresh_plan.drive_import_plans.is_empty());
    assert_eq!(
        refresh_plan.outbox_events[0].event_type,
        "video.generation.succeeded"
    );

    let persistence_plan = plan_video_generation_refresh_persistence_plan(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: VideoGenerationActor::User {
                user_id: "user-success".to_string(),
            },
        },
        "video-generation-succeeded-empty",
        "success_scene",
        Some("kling-v2".to_string()),
        succeeded,
    )
    .expect("success persistence plan should build");

    assert_eq!(
        persistence_plan.runtime_status,
        VideoGenerationRuntimeStatus::Succeeded
    );
    assert_eq!(persistence_plan.drive_sync_status, "imported");
    assert!(persistence_plan
        .repository_methods
        .contains(&"mark_generation_succeeded".to_string()));
}

#[test]
fn plans_drive_import_completion_as_generation_succeeded() {
    let persistence_plan = plan_video_generation_drive_import_completion_persistence_plan(
        "video-generation-drive-complete",
        "vidu",
        Some("task-drive-complete".to_string()),
        Some("completed".to_string()),
        2,
    )
    .expect("drive completion persistence plan should build");

    assert_eq!(
        persistence_plan.runtime_status,
        VideoGenerationRuntimeStatus::Succeeded
    );
    assert_eq!(persistence_plan.drive_sync_status, "imported");
    assert_eq!(persistence_plan.provider_code, "vidu");
    assert_eq!(
        persistence_plan.provider_task_id.as_deref(),
        Some("task-drive-complete")
    );
    assert!(persistence_plan.output_rows.is_empty());
    assert!(persistence_plan
        .repository_methods
        .contains(&"mark_drive_imported".to_string()));
    assert!(persistence_plan
        .repository_methods
        .contains(&"mark_generation_succeeded".to_string()));

    let steps = plan_video_generation_drive_import_completion_runtime_steps(
        "video-generation-drive-complete",
        2,
    )
    .expect("drive completion runtime steps should build");

    assert_eq!(
        steps,
        vec![
            VideoGenerationRuntimeStep::MarkDriveImported { output_count: 2 },
            VideoGenerationRuntimeStep::MarkGenerationSucceeded,
            VideoGenerationRuntimeStep::PersistOutboxEvent {
                event_type: "video.generation.succeeded".to_string(),
            },
        ],
    );
}

#[test]
fn plans_executable_runtime_steps_for_generation_create_flow() {
    let steps = plan_video_generation_create_runtime_steps(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-runtime-001",
        VideoGenerationCreateCommand {
            prompt: "Product reveal".to_string(),
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
            webhook_url: Some("https://app.example.com/hooks/video".to_string()),
            idempotency_key: Some("idem-runtime-001".to_string()),
        },
    )
    .expect("runtime steps should plan");

    assert_eq!(
        steps,
        vec![
            VideoGenerationRuntimeStep::CreateGenerationRecord,
            VideoGenerationRuntimeStep::DispatchProviderGeneration {
                provider_id: String::new(),
                provider_code: "vidu".to_string(),
            },
            VideoGenerationRuntimeStep::PersistProviderSubmission,
            VideoGenerationRuntimeStep::AwaitProviderWebhook,
            VideoGenerationRuntimeStep::ScheduleProviderPolling,
            VideoGenerationRuntimeStep::PersistOutboxEvent {
                event_type: "video.generation.created".to_string(),
            },
        ],
    );
}

#[test]
fn rejects_create_runtime_steps_when_vidu_image_operation_has_no_source_image() {
    let error = plan_video_generation_create_runtime_steps(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-runtime-invalid",
        VideoGenerationCreateCommand {
            prompt: "Product reveal".to_string(),
            negative_prompt: None,
            scene: "product_reveal".to_string(),
            provider_code: Some("vidu".to_string()),
            operation: Some("image_to_video".to_string()),
            model: Some("vidu2.0".to_string()),
            resolution: Some("1080p".to_string()),
            aspect_ratio: Some("16:9".to_string()),
            duration_seconds: Some(8),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: Some("normal".to_string()),
            webhook_url: Some("https://app.example.com/hooks/video".to_string()),
            idempotency_key: Some("idem-runtime-invalid".to_string()),
        },
    )
    .expect_err("service runtime planning must propagate provider dispatch validation");

    assert_eq!(error, "vidu image_to_video requires one source image");
}

#[test]
fn rejects_create_runtime_steps_for_unregistered_vendor() {
    let error = plan_video_generation_create_runtime_steps(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-runtime-unsupported",
        VideoGenerationCreateCommand {
            prompt: "Future provider task".to_string(),
            negative_prompt: None,
            scene: "provider_native".to_string(),
            provider_code: Some("runway".to_string()),
            operation: None,
            model: Some("runway-gen3".to_string()),
            resolution: Some("1080p".to_string()),
            aspect_ratio: Some("16:9".to_string()),
            duration_seconds: Some(5),
            start_image: None,
            end_image: None,
            reference_images: vec![],
            motion_strength: None,
            webhook_url: None,
            idempotency_key: Some("idem-runtime-unsupported".to_string()),
        },
    )
    .expect_err("runtime steps must not claim unsupported provider operations are executable");

    assert_eq!(
        error,
        "video generation vendor is not supported by a registered provider"
    );
}

#[test]
fn rejects_create_service_flow_for_unregistered_vendor() {
    let error = plan_video_generation_create_service_flow(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-flow-unsupported",
        unsupported_provider_command(),
        None,
    )
    .expect_err("service flow must not plan unsupported provider operations");

    assert_eq!(
        error,
        "video generation vendor is not supported by a registered provider"
    );
}

#[test]
fn rejects_create_persistence_plan_for_unregistered_vendor() {
    let error = plan_video_generation_create_persistence_plan(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: VideoGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "video-generation-persist-unsupported",
        unsupported_provider_command(),
        None,
    )
    .expect_err("persistence plan must not plan unsupported provider operations");

    assert_eq!(
        error,
        "video generation vendor is not supported by a registered provider"
    );
}

#[test]
fn plans_executable_runtime_steps_for_refresh_outputs_and_drive_sync() {
    let result = normalize_provider_task_video_generation_result(
        "kling",
        ProviderTaskSnapshot {
            task_id: Some("task-runtime-002".to_string()),
            id: None,
            status: Some("completed".to_string()),
            state: None,
            model: Some("kling-v2".to_string()),
            videos: vec![
                ProviderGeneratedVideoAsset {
                    id: Some("asset-runtime-0".to_string()),
                    uri: None,
                    url: Some("https://provider.example.com/runtime-0.mp4".to_string()),
                    mime_type: Some("video/mp4".to_string()),
                    width: Some(1280),
                    height: Some(720),
                    duration_seconds: Some(5),
                },
                ProviderGeneratedVideoAsset {
                    id: Some("asset-runtime-1".to_string()),
                    uri: None,
                    url: Some("https://provider.example.com/runtime-1.mp4".to_string()),
                    mime_type: Some("video/mp4".to_string()),
                    width: Some(1280),
                    height: Some(720),
                    duration_seconds: Some(5),
                },
            ],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let steps = plan_video_generation_refresh_runtime_steps(
        VideoGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: VideoGenerationActor::Anonymous {
                anonymous_id: "anon-runtime-002".to_string(),
            },
        },
        "video-generation-runtime-002",
        "anonymous_tryout",
        Some("kling-v2".to_string()),
        result,
    )
    .expect("runtime refresh steps should plan");

    assert_eq!(
        steps,
        vec![
            VideoGenerationRuntimeStep::PersistProviderSubmission,
            VideoGenerationRuntimeStep::PersistDriveImportPlan { output_count: 2 },
            VideoGenerationRuntimeStep::PrepareDriveUpload { output_count: 2 },
            VideoGenerationRuntimeStep::PersistOutboxEvent {
                event_type: "video.generation.outputs_ready".to_string(),
            },
        ],
    );
}

fn unsupported_provider_command() -> VideoGenerationCreateCommand {
    VideoGenerationCreateCommand {
        prompt: "Future provider task".to_string(),
        negative_prompt: None,
        scene: "provider_native".to_string(),
        provider_code: Some("runway".to_string()),
        operation: None,
        model: Some("runway-gen3".to_string()),
        resolution: Some("1080p".to_string()),
        aspect_ratio: Some("16:9".to_string()),
        duration_seconds: Some(5),
        start_image: None,
        end_image: None,
        reference_images: vec![],
        motion_strength: None,
        webhook_url: None,
        idempotency_key: Some("idem-unsupported".to_string()),
    }
}
