use crate::{
    GenerateVideoInput, InMemoryVideoGenerationMcpTaskStore, McpToolError,
    VideoGenerationMcpTaskContext, VideoGenerationMcpTaskStore, VideoGenerationResult,
    VideoTaskInput,
};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, ErrorData, GetPromptRequestParams, GetPromptResult, Implementation,
        ListPromptsResult, ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams,
        ReadResourceResult, ServerCapabilities, ServerInfo, Tool,
    },
    service::RequestContext,
    tool, tool_handler, tool_router, Json, RoleServer, ServerHandler,
};
use sdkwork_video_generation_service::{
    VideoGenerationProviderDescriptor, VideoGenerationServicePort,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct VideoGenerationMcpService {
    generation_service: Arc<dyn VideoGenerationServicePort>,
    task_store: Arc<dyn VideoGenerationMcpTaskStore>,
    tool_router: ToolRouter<Self>,
}

impl VideoGenerationMcpService {
    pub fn new(generation_service: Arc<dyn VideoGenerationServicePort>) -> Self {
        Self::with_task_store(
            generation_service,
            InMemoryVideoGenerationMcpTaskStore::shared_default(),
        )
    }
    pub fn with_task_store(
        generation_service: Arc<dyn VideoGenerationServicePort>,
        task_store: Arc<dyn VideoGenerationMcpTaskStore>,
    ) -> Self {
        Self {
            generation_service,
            task_store,
            tool_router: Self::tool_router(),
        }
    }
    pub fn tools(&self) -> Vec<Tool> {
        self.tool_router.list_all()
    }
    pub fn provider_descriptors(&self) -> Vec<VideoGenerationProviderDescriptor> {
        self.generation_service.provider_descriptors()
    }
    fn task_context(&self, handle: &str) -> Result<VideoGenerationMcpTaskContext, McpToolError> {
        let handle = handle.trim();
        if handle.is_empty() {
            return Err(McpToolError::invalid_request("taskHandle is required"));
        }
        self.task_store
            .load(handle)?
            .ok_or_else(|| McpToolError::task_not_found(handle))
    }
}

#[tool_router]
impl VideoGenerationMcpService {
    #[tool(
        name = "video.generate",
        description = "Generate video through the unified video generation service."
    )]
    async fn generate(
        &self,
        Parameters(input): Parameters<GenerateVideoInput>,
    ) -> Result<Json<VideoGenerationResult>, Json<McpToolError>> {
        let submission = self
            .generation_service
            .generate(input.try_into().map_err(Json)?)
            .await
            .map_err(|error| Json(error.into()))?;
        let task_handle = match submission.result.provider_task_id.as_deref() {
            Some(provider_task_id) => Some(
                self.task_store
                    .save(VideoGenerationMcpTaskContext {
                        dispatch_plan: submission.dispatch_plan.clone(),
                        provider_task_id: provider_task_id.into(),
                    })
                    .map_err(Json)?,
            ),
            None => None,
        };
        Ok(Json(VideoGenerationResult::from_submission(
            &submission,
            task_handle,
        )))
    }
    #[tool(
        name = "video.retrieve",
        description = "Retrieve a video generation task by the task handle returned from video.generate."
    )]
    async fn retrieve(
        &self,
        Parameters(input): Parameters<VideoTaskInput>,
    ) -> Result<Json<VideoGenerationResult>, Json<McpToolError>> {
        let context = self.task_context(&input.task_handle).map_err(Json)?;
        let result = self
            .generation_service
            .retrieve(&context.dispatch_plan, &context.provider_task_id)
            .await
            .map_err(|error| Json(error.into()))?;
        Ok(Json(VideoGenerationResult::from_normalized(
            &result,
            Some(input.task_handle),
        )))
    }
    #[tool(
        name = "video.cancel",
        description = "Cancel a video generation task by the task handle returned from video.generate."
    )]
    async fn cancel(
        &self,
        Parameters(input): Parameters<VideoTaskInput>,
    ) -> Result<Json<VideoGenerationResult>, Json<McpToolError>> {
        let context = self.task_context(&input.task_handle).map_err(Json)?;
        let result = self
            .generation_service
            .cancel(&context.dispatch_plan, &context.provider_task_id)
            .await
            .map_err(|error| Json(error.into()))?;
        Ok(Json(VideoGenerationResult::from_normalized(
            &result,
            Some(input.task_handle),
        )))
    }
    #[tool(
        name = "video.capabilities",
        description = "List registered video generation vendors and capabilities."
    )]
    async fn capabilities(&self) -> CallToolResult {
        CallToolResult::structured(crate::catalog::catalog(self.provider_descriptors()))
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for VideoGenerationMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().enable_resources().enable_prompts().build())
            .with_server_info(Implementation::new("sdkwork-video-generation-mcp-service", env!("CARGO_PKG_VERSION")))
            .with_instructions("Use provider-neutral video generation tools and inspect capability resources before setting vendor-specific parameters.")
    }
    async fn list_resources(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        Ok(crate::catalog::resources())
    }
    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        crate::catalog::read(&request.uri, self.provider_descriptors())
            .map(|content| ReadResourceResult::new(vec![content]))
            .ok_or_else(|| ErrorData::resource_not_found("video MCP resource was not found", None))
    }
    async fn list_prompts(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, ErrorData> {
        Ok(crate::catalog::prompts())
    }
    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, ErrorData> {
        if request.name == crate::catalog::GENERATION_PROMPT {
            Ok(crate::catalog::prompt())
        } else {
            Err(ErrorData::invalid_params(
                "video MCP prompt was not found",
                None,
            ))
        }
    }
}
