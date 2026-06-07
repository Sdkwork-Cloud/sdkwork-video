use std::collections::{BTreeMap, BTreeSet};

use sdkwork_drive_product::{
    domain::space::DriveSpaceType,
    uploader::{PrepareUploaderUploadCommand, UploaderActor, UploaderRetention, UploaderTarget},
};

use crate::{
    identity::{GENERATED_VIDEO_DEFAULT_CHUNK_SIZE_BYTES, VIDEO_WORKSPACE},
    media_format::video_file_extension_for_mime,
    models::{
        DriveBackedVideoMediaResource, DriveGeneratedVideoContext, DriveGeneratedVideoImportPlan,
        GeneratedVideoOutput, MediaAiProvenance, VideoGenerationActor,
    },
    text::{require_trimmed, stable_identifier_suffix},
};

pub fn plan_drive_import_for_generated_video_outputs(
    context: DriveGeneratedVideoContext,
    outputs: Vec<GeneratedVideoOutput>,
) -> Result<Vec<DriveGeneratedVideoImportPlan>, &'static str> {
    require_trimmed(&context.tenant_id, "generated video tenant is required")?;
    let generation_id = require_trimmed(
        &context.generation_id,
        "generated video generation_id is required",
    )?;
    let provider_code = require_trimmed(
        &context.provider_code,
        "generated video provider_code is required",
    )?;
    let scene = require_trimmed(&context.scene, "generated video scene is required")?;
    if outputs.is_empty() {
        return Err("generated video outputs are required");
    }

    let mut output_indexes = BTreeSet::new();
    for output in &outputs {
        if output.output_index < 0 {
            return Err("generated video output_index must be non-negative");
        }
        if !output_indexes.insert(output.output_index) {
            return Err("generated video output_index must be unique");
        }
    }

    let owner = resolve_drive_owner(&context.actor)?;
    let drive_space_type = DriveSpaceType::AiGenerated.as_str().to_string();
    let owner_suffix = stable_identifier_suffix(&owner.owner_subject_id);
    let drive_space_id = format!(
        "space-ai-generated-{}-{}",
        owner.owner_subject_type, owner_suffix
    );

    let mut plans = Vec::with_capacity(outputs.len());
    for output in outputs {
        let mime_type = output
            .mime_type
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let file_name = output
            .file_name
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| {
                format!(
                    "{}-{}.{}",
                    stable_identifier_suffix(generation_id),
                    output.output_index,
                    video_file_extension_for_mime(mime_type)
                )
            });
        let drive_node_id = format!(
            "node-ai-generated-video-{}-{}",
            stable_identifier_suffix(generation_id),
            output.output_index
        );
        let drive_uri = format!("drive://spaces/{drive_space_id}/nodes/{drive_node_id}");
        let drive_upload_task_id = format!(
            "video-generation-{}-{}",
            stable_identifier_suffix(generation_id),
            output.output_index
        );
        let metadata = media_resource_metadata(MediaResourceMetadataInput {
            drive_space_type: &drive_space_type,
            drive_space_id: &drive_space_id,
            drive_node_id: &drive_node_id,
            scene,
            provider_code,
            generation_id,
            organization_id: &context.organization_id,
            output: &output,
        });

        let media_resource = DriveBackedVideoMediaResource {
            id: drive_node_id.clone(),
            kind: "video".to_string(),
            source: "drive".to_string(),
            uri: drive_uri.clone(),
            url: None,
            file_name: Some(file_name),
            mime_type: mime_type
                .map(str::to_string)
                .or_else(|| Some("video/mp4".to_string())),
            size_bytes: output.size_bytes.map(|value| value.to_string()),
            width: output.width,
            height: output.height,
            duration_seconds: output.duration_seconds,
            object_blob_id: None,
            ai: MediaAiProvenance {
                provenance: "generated".to_string(),
                provider: Some(provider_code.to_string()),
                model: context
                    .model
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_string),
                generation_task_id: Some(generation_id.to_string()),
                moderation_status: "unknown".to_string(),
            },
            metadata,
        };

        plans.push(DriveGeneratedVideoImportPlan {
            generation_id: generation_id.to_string(),
            output_index: output.output_index,
            scene: scene.to_string(),
            provider_code: provider_code.to_string(),
            provider_asset_id: output.provider_asset_id,
            provider_uri: output.provider_uri,
            provider_url: output.provider_url,
            drive_space_type: drive_space_type.clone(),
            drive_owner_subject_type: owner.owner_subject_type.clone(),
            drive_owner_subject_id: owner.owner_subject_id.clone(),
            drive_actor_type: owner.actor_type.clone(),
            drive_actor_id: owner.actor_id.clone(),
            drive_space_id: drive_space_id.clone(),
            drive_parent_node_id: None,
            drive_node_id,
            drive_uri,
            drive_upload_profile_code: "video".to_string(),
            drive_upload_task_id,
            media_resource,
        });
    }

    Ok(plans)
}

pub fn build_drive_uploader_command_for_generated_video_output(
    plan: &DriveGeneratedVideoImportPlan,
    tenant_id: impl AsRef<str>,
    organization_id: Option<&str>,
    operator_id: impl AsRef<str>,
    now_epoch_ms: i64,
) -> Result<PrepareUploaderUploadCommand, &'static str> {
    let tenant_id = require_trimmed(tenant_id.as_ref(), "generated video tenant is required")?;
    let operator_id = require_trimmed(
        operator_id.as_ref(),
        "generated video drive operator_id is required",
    )?;
    if now_epoch_ms <= 0 {
        return Err("generated video drive now_epoch_ms must be greater than 0");
    }
    let scene = require_trimmed(&plan.scene, "generated video scene is required")?;
    let upload_id = plan
        .drive_node_id
        .strip_prefix("node-")
        .unwrap_or(&plan.drive_node_id)
        .to_string();
    let file_name = plan
        .media_resource
        .file_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or("generated video drive file_name is required")?
        .to_string();
    let content_type = plan
        .media_resource
        .mime_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
        .unwrap_or_else(|| "video/mp4".to_string());

    Ok(PrepareUploaderUploadCommand {
        id: upload_id,
        task_id: plan.drive_upload_task_id.clone(),
        tenant_id: tenant_id.to_string(),
        organization_id: organization_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string),
        actor: uploader_actor_for_drive_plan(plan)?,
        app_id: VIDEO_WORKSPACE.to_string(),
        app_resource_type: "ai_video_generation_output".to_string(),
        app_resource_id: format!("{}:{}", plan.generation_id, plan.output_index),
        scene: Some(scene.to_string()),
        source: Some("ai_generated".to_string()),
        upload_profile_code: "video".to_string(),
        file_fingerprint: format!(
            "sdkwork-video:ai-generated:{}:{}",
            stable_identifier_suffix(&plan.generation_id),
            plan.output_index
        ),
        original_file_name: file_name,
        content_type,
        content_length: parse_content_length(plan.media_resource.size_bytes.as_deref())?,
        chunk_size_bytes: GENERATED_VIDEO_DEFAULT_CHUNK_SIZE_BYTES,
        target: UploaderTarget::AiGeneratedSpace {
            parent_node_id: plan.drive_parent_node_id.clone(),
        },
        retention: UploaderRetention::LongTerm,
        operator_id: operator_id.to_string(),
        now_epoch_ms,
    })
}

struct MediaResourceMetadataInput<'a> {
    drive_space_type: &'a str,
    drive_space_id: &'a str,
    drive_node_id: &'a str,
    scene: &'a str,
    provider_code: &'a str,
    generation_id: &'a str,
    organization_id: &'a Option<String>,
    output: &'a GeneratedVideoOutput,
}

fn media_resource_metadata(input: MediaResourceMetadataInput<'_>) -> BTreeMap<String, String> {
    let mut metadata = BTreeMap::new();
    metadata.insert("spaceType".to_string(), input.drive_space_type.to_string());
    metadata.insert("spaceId".to_string(), input.drive_space_id.to_string());
    metadata.insert("nodeId".to_string(), input.drive_node_id.to_string());
    metadata.insert("scene".to_string(), input.scene.to_string());
    metadata.insert("provider".to_string(), input.provider_code.to_string());
    metadata.insert("generationId".to_string(), input.generation_id.to_string());
    metadata.insert(
        "outputIndex".to_string(),
        input.output.output_index.to_string(),
    );
    if let Some(organization_id) = input
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        metadata.insert("organizationId".to_string(), organization_id.to_string());
    }
    if let Some(provider_asset_id) = input
        .output
        .provider_asset_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        metadata.insert("providerAssetId".to_string(), provider_asset_id.to_string());
    }
    if let Some(provider_uri) = input
        .output
        .provider_uri
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        metadata.insert("providerUri".to_string(), provider_uri.to_string());
    }
    metadata
}

struct DriveOwner {
    owner_subject_type: String,
    owner_subject_id: String,
    actor_type: String,
    actor_id: String,
}

fn resolve_drive_owner(actor: &VideoGenerationActor) -> Result<DriveOwner, &'static str> {
    match actor {
        VideoGenerationActor::User { user_id } => {
            let user_id = require_trimmed(user_id, "generated video user_id is required")?;
            Ok(DriveOwner {
                owner_subject_type: "user".to_string(),
                owner_subject_id: user_id.to_string(),
                actor_type: "user".to_string(),
                actor_id: user_id.to_string(),
            })
        }
        VideoGenerationActor::Anonymous { anonymous_id } => {
            let anonymous_id =
                require_trimmed(anonymous_id, "generated video anonymous_id is required")?;
            Ok(DriveOwner {
                owner_subject_type: "app".to_string(),
                owner_subject_id: format!("app:{VIDEO_WORKSPACE}:anonymous"),
                actor_type: "anonymous".to_string(),
                actor_id: anonymous_id.to_string(),
            })
        }
        VideoGenerationActor::System { operator_id } => {
            let operator_id =
                require_trimmed(operator_id, "generated video operator_id is required")?;
            Ok(DriveOwner {
                owner_subject_type: "app".to_string(),
                owner_subject_id: format!("app:{VIDEO_WORKSPACE}:system"),
                actor_type: "system".to_string(),
                actor_id: operator_id.to_string(),
            })
        }
    }
}

fn uploader_actor_for_drive_plan(
    plan: &DriveGeneratedVideoImportPlan,
) -> Result<UploaderActor, &'static str> {
    match plan.drive_actor_type.as_str() {
        "user" => Ok(UploaderActor::User {
            user_id: require_trimmed(
                &plan.drive_actor_id,
                "generated video drive user_id is required",
            )?
            .to_string(),
        }),
        "anonymous" => Ok(UploaderActor::Anonymous {
            anonymous_id: require_trimmed(
                &plan.drive_actor_id,
                "generated video drive anonymous_id is required",
            )?
            .to_string(),
        }),
        "system" => Ok(UploaderActor::System {
            operator_id: require_trimmed(
                &plan.drive_actor_id,
                "generated video drive system operator_id is required",
            )?
            .to_string(),
        }),
        _ => Err("generated video drive actor_type is not supported"),
    }
}

fn parse_content_length(value: Option<&str>) -> Result<i64, &'static str> {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(0);
    };
    let parsed = value
        .parse::<i64>()
        .map_err(|_| "generated video size_bytes must be a non-negative integer")?;
    if parsed < 0 {
        Err("generated video size_bytes must be a non-negative integer")
    } else {
        Ok(parsed)
    }
}
