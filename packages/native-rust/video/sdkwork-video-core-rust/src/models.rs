use std::collections::BTreeMap;

use crate::status::{VideoGenerationRuntimeStatus, VideoProviderOperation, VideoProviderTaskMode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoGenerationCreateCommand {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub scene: String,
    pub provider_code: Option<String>,
    pub operation: Option<String>,
    pub model: Option<String>,
    pub resolution: Option<String>,
    pub aspect_ratio: Option<String>,
    pub duration_seconds: Option<i32>,
    pub start_image: Option<String>,
    pub end_image: Option<String>,
    pub reference_images: Vec<String>,
    pub motion_strength: Option<String>,
    pub webhook_url: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoProviderDispatchPlan {
    pub provider_code: String,
    pub provider_operation: VideoProviderOperation,
    pub task_mode: VideoProviderTaskMode,
    pub claw_router_api_path: &'static str,
    pub claw_router_sdk_resource: &'static str,
    pub claw_router_sdk_method: &'static str,
    pub scene: String,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub model: Option<String>,
    pub resolution: Option<String>,
    pub aspect_ratio: Option<String>,
    pub duration_seconds: Option<i32>,
    pub start_image: Option<String>,
    pub end_image: Option<String>,
    pub source_images: Vec<String>,
    pub motion_strength: Option<String>,
    pub callback_url: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderGeneratedVideoAsset {
    pub id: Option<String>,
    pub uri: Option<String>,
    pub url: Option<String>,
    pub mime_type: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration_seconds: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderTaskErrorSnapshot {
    pub code: Option<String>,
    pub message: Option<String>,
    pub error_type: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderTaskSnapshot {
    pub task_id: Option<String>,
    pub id: Option<String>,
    pub status: Option<String>,
    pub state: Option<String>,
    pub model: Option<String>,
    pub videos: Vec<ProviderGeneratedVideoAsset>,
    pub error: Option<ProviderTaskErrorSnapshot>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedProviderVideoGenerationResult {
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub provider_state: Option<String>,
    pub status: VideoGenerationRuntimeStatus,
    pub provider_terminal: bool,
    pub ready_for_drive_import: bool,
    pub outputs: Vec<GeneratedVideoOutput>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VideoGenerationActor {
    Anonymous { anonymous_id: String },
    User { user_id: String },
    System { operator_id: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveGeneratedVideoContext {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub generation_id: String,
    pub provider_code: String,
    pub model: Option<String>,
    pub scene: String,
    pub actor: VideoGenerationActor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedVideoOutput {
    pub output_index: i32,
    pub provider_asset_id: Option<String>,
    pub provider_uri: Option<String>,
    pub provider_url: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaAiProvenance {
    pub provenance: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub generation_task_id: Option<String>,
    pub moderation_status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveBackedVideoMediaResource {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub uri: String,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub object_blob_id: Option<String>,
    pub ai: MediaAiProvenance,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveGeneratedVideoImportPlan {
    pub generation_id: String,
    pub output_index: i32,
    pub scene: String,
    pub provider_code: String,
    pub provider_asset_id: Option<String>,
    pub provider_uri: Option<String>,
    pub provider_url: Option<String>,
    pub drive_space_type: String,
    pub drive_owner_subject_type: String,
    pub drive_owner_subject_id: String,
    pub drive_actor_type: String,
    pub drive_actor_id: String,
    pub drive_space_id: String,
    pub drive_parent_node_id: Option<String>,
    pub drive_node_id: String,
    pub drive_uri: String,
    pub drive_upload_profile_code: String,
    pub drive_upload_task_id: String,
    pub media_resource: DriveBackedVideoMediaResource,
}
