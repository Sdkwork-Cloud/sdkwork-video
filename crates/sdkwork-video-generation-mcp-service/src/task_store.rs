use std::{
    collections::{HashMap, VecDeque},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use crate::McpToolError;
use sdkwork_video_generation_service::VideoProviderDispatchPlan;

#[derive(Clone, Debug)]
pub struct VideoGenerationMcpTaskContext {
    pub dispatch_plan: VideoProviderDispatchPlan,
    pub provider_task_id: String,
}

pub trait VideoGenerationMcpTaskStore: Send + Sync {
    fn save(&self, context: VideoGenerationMcpTaskContext) -> Result<String, McpToolError>;
    fn load(&self, handle: &str) -> Result<Option<VideoGenerationMcpTaskContext>, McpToolError>;
}

pub struct InMemoryVideoGenerationMcpTaskStore {
    capacity: usize,
    sequence: AtomicU64,
    state: Mutex<TaskStoreState>,
}

#[derive(Default)]
struct TaskStoreState {
    order: VecDeque<String>,
    contexts: HashMap<String, VideoGenerationMcpTaskContext>,
}

impl InMemoryVideoGenerationMcpTaskStore {
    pub const DEFAULT_CAPACITY: usize = 2_048;

    pub fn new(capacity: usize) -> Result<Self, McpToolError> {
        if capacity == 0 {
            return Err(McpToolError::invalid_request(
                "video MCP task store capacity must be greater than zero",
            ));
        }
        Ok(Self {
            capacity,
            sequence: AtomicU64::new(1),
            state: Mutex::new(TaskStoreState::default()),
        })
    }

    pub fn shared_default() -> Arc<dyn VideoGenerationMcpTaskStore> {
        Arc::new(Self::new(Self::DEFAULT_CAPACITY).expect("valid video MCP task store capacity"))
    }
}

impl VideoGenerationMcpTaskStore for InMemoryVideoGenerationMcpTaskStore {
    fn save(&self, context: VideoGenerationMcpTaskContext) -> Result<String, McpToolError> {
        let handle = format!(
            "video-task-{}",
            self.sequence.fetch_add(1, Ordering::Relaxed)
        );
        let mut state = self
            .state
            .lock()
            .map_err(|_| McpToolError::store_unavailable())?;
        while state.contexts.len() >= self.capacity {
            if let Some(expired) = state.order.pop_front() {
                state.contexts.remove(&expired);
            }
        }
        state.order.push_back(handle.clone());
        state.contexts.insert(handle.clone(), context);
        Ok(handle)
    }

    fn load(&self, handle: &str) -> Result<Option<VideoGenerationMcpTaskContext>, McpToolError> {
        Ok(self
            .state
            .lock()
            .map_err(|_| McpToolError::store_unavailable())?
            .contexts
            .get(handle.trim())
            .cloned())
    }
}
