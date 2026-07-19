use crate::VideoGenerationMcpService;
use rmcp::{
    service::{RunningService, ServerInitializeError},
    transport::streamable_http_server::{
        session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
    },
    RoleServer, ServiceExt,
};
use std::sync::Arc;

pub type VideoGenerationMcpHttpService =
    StreamableHttpService<VideoGenerationMcpService, LocalSessionManager>;
pub fn streamable_http_service(
    service: VideoGenerationMcpService,
    config: StreamableHttpServerConfig,
) -> VideoGenerationMcpHttpService {
    StreamableHttpService::new(
        move || Ok(service.clone()),
        Arc::new(LocalSessionManager::default()),
        config,
    )
}
pub async fn serve_stdio(
    service: VideoGenerationMcpService,
) -> Result<RunningService<RoleServer, VideoGenerationMcpService>, ServerInitializeError> {
    service.serve(rmcp::transport::stdio()).await
}
