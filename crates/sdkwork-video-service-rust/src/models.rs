use sdkwork_video_core::{
    DriveGeneratedVideoImportPlan, NormalizedProviderVideoGenerationResult, VideoGenerationActor,
    VideoGenerationRuntimeStatus, VideoProviderDispatchPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationScope {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub actor: VideoGenerationActor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationRecord {
    pub generation_id: String,
    pub status: VideoGenerationRuntimeStatus,
    pub scene: String,
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub drive_space_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationProviderSubmission {
    pub generation_id: String,
    pub dispatch_plan: VideoProviderDispatchPlan,
    pub normalized_result: Option<NormalizedProviderVideoGenerationResult>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationServicePlan {
    pub record: VideoGenerationRecord,
    pub dispatch: VideoGenerationProviderSubmission,
    pub drive_import_plans: Vec<DriveGeneratedVideoImportPlan>,
    pub outbox_events: Vec<VideoGenerationOutboxEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationOutboxEvent {
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationOutputPersistenceRow {
    pub output_index: i32,
    pub media_kind: String,
    pub scene: String,
    pub provider_code: String,
    pub provider_asset_id: Option<String>,
    pub provider_uri: Option<String>,
    pub provider_url: Option<String>,
    pub drive_space_type: String,
    pub drive_space_id: String,
    pub drive_parent_node_id: Option<String>,
    pub drive_node_id: String,
    pub drive_uri: String,
    pub resource_snapshot_id: String,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub sync_status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationPersistencePlan {
    pub generation_id: String,
    pub runtime_status: VideoGenerationRuntimeStatus,
    pub drive_sync_status: String,
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub output_rows: Vec<VideoGenerationOutputPersistenceRow>,
    pub repository_methods: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationWebhookEnvelope {
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub external_event_id: Option<String>,
    pub event_type: String,
    pub payload_hash: String,
    pub normalized_result: NormalizedProviderVideoGenerationResult,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationRefreshPlan {
    pub generation_id: String,
    pub status: VideoGenerationRuntimeStatus,
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub drive_import_plans: Vec<DriveGeneratedVideoImportPlan>,
    pub outbox_events: Vec<VideoGenerationOutboxEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VideoGenerationRuntimeStep {
    CreateGenerationRecord,
    DispatchProviderGeneration {
        provider_id: String,
        provider_code: String,
    },
    PersistProviderSubmission,
    ScheduleProviderPolling,
    AwaitProviderWebhook,
    PersistDriveImportPlan {
        output_count: i32,
    },
    PrepareDriveUpload {
        output_count: i32,
    },
    MarkDriveImported {
        output_count: i32,
    },
    MarkGenerationSucceeded,
    PersistOutboxEvent {
        event_type: String,
    },
}
