use rmcp::schemars::JsonSchema;
use sdkwork_video_generation_service::{
    NormalizedProviderVideoGenerationResult, VideoGenerationCommand, VideoGenerationModelSelection,
    VideoGenerationVendorParameters, VideoProviderSubmission, VideoVendorId,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::McpToolError;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorParametersInput {
    pub schema: String,
    pub values: Value,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateVideoInput {
    pub vendor: String,
    #[serde(default)]
    pub operation: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    pub prompt: String,
    #[serde(default)]
    pub negative_prompt: Option<String>,
    #[serde(default = "default_scene")]
    pub scene: String,
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub aspect_ratio: Option<String>,
    #[serde(default)]
    pub duration_seconds: Option<i32>,
    #[serde(default)]
    pub start_image: Option<String>,
    #[serde(default)]
    pub end_image: Option<String>,
    #[serde(default)]
    pub reference_images: Vec<String>,
    #[serde(default)]
    pub motion_strength: Option<String>,
    #[serde(default)]
    pub callback_url: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub vendor_parameters: Option<VendorParametersInput>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoTaskInput {
    pub task_handle: String,
}

#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoGenerationResult {
    pub vendor: String,
    pub task_handle: Option<String>,
    pub status: String,
    pub terminal: bool,
    pub outputs: Vec<VideoOutput>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoOutput {
    pub output_index: i32,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
}

impl TryFrom<GenerateVideoInput> for VideoGenerationCommand {
    type Error = McpToolError;
    fn try_from(input: GenerateVideoInput) -> Result<Self, Self::Error> {
        let vendor = VideoVendorId::new(input.vendor).map_err(McpToolError::invalid_request)?;
        let model = match input.model.as_deref().map(str::trim) {
            Some(value) if !value.is_empty() => VideoGenerationModelSelection::named(value)
                .map_err(McpToolError::invalid_request)?,
            _ => VideoGenerationModelSelection::VendorDefault,
        };
        Ok(Self {
            vendor,
            operation: input.operation,
            model,
            prompt: input.prompt,
            negative_prompt: input.negative_prompt,
            scene: input.scene,
            resolution: input.resolution,
            aspect_ratio: input.aspect_ratio,
            duration_seconds: input.duration_seconds,
            start_image: input.start_image,
            end_image: input.end_image,
            reference_images: input.reference_images,
            motion_strength: input.motion_strength,
            callback_url: input.callback_url,
            idempotency_key: input.idempotency_key,
            vendor_parameters: input.vendor_parameters.map(|parameters| {
                VideoGenerationVendorParameters {
                    schema: parameters.schema,
                    values: parameters.values,
                }
            }),
        })
    }
}

impl VideoGenerationResult {
    pub(crate) fn from_submission(
        submission: &VideoProviderSubmission,
        task_handle: Option<String>,
    ) -> Self {
        Self::from_normalized(&submission.result, task_handle)
    }
    pub(crate) fn from_normalized(
        result: &NormalizedProviderVideoGenerationResult,
        task_handle: Option<String>,
    ) -> Self {
        Self {
            vendor: result.provider_code.clone(),
            task_handle,
            status: result.status.as_str().into(),
            terminal: result.provider_terminal,
            outputs: result
                .outputs
                .iter()
                .map(|output| VideoOutput {
                    output_index: output.output_index,
                    url: output.provider_url.clone(),
                    file_name: output.file_name.clone(),
                    mime_type: output.mime_type.clone(),
                    size_bytes: output.size_bytes,
                    width: output.width,
                    height: output.height,
                    duration_seconds: output.duration_seconds,
                })
                .collect(),
            error_code: result.error_code.clone(),
            error_message: result.error_message.clone(),
        }
    }
}

fn default_scene() -> String {
    "agent.video.generation".into()
}
