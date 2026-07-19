mod catalog;
mod dto;
mod error;
mod handler;
mod task_store;
mod transport;

pub use dto::*;
pub use error::McpToolError;
pub use handler::VideoGenerationMcpService;
pub use task_store::{
    InMemoryVideoGenerationMcpTaskStore, VideoGenerationMcpTaskContext, VideoGenerationMcpTaskStore,
};
pub use transport::{serve_stdio, streamable_http_service, VideoGenerationMcpHttpService};
