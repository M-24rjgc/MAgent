// 抑制开发期间的未使用代码警告
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

pub mod agent_runtime;
pub mod data_structures;
pub mod state_machine;

pub use data_structures::*;

#[derive(Clone)]
pub struct AppState {
    pub agent_runtime: Arc<crate::agents::AgentRuntime>,
    pub config: Arc<RwLock<crate::config::AppConfig>>,
    pub event_bus: mpsc::UnboundedSender<SystemEvent>,
    pub storage: Arc<crate::storage::DataStore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    ConceptOptimizationStarted {
        session_id: Uuid,
    },
    IterationCompleted {
        session_id: Uuid,
        version: u32,
    },
    OptimizationCompleted {
        session_id: Uuid,
    },
    ErrorOccurred {
        session_id: Option<Uuid>,
        error: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSession {
    pub id: Uuid,
    pub idea_seed: IdeaSeed,
    pub current_state: SessionState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionState {
    Initializing,
    Clarifying,
    Clarified,
    AdvIterating(u32), // iteration number
    Verified,
    Formatting,
    Done,
    Error(String),
}
