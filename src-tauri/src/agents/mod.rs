// 抑制开发期间的未使用代码警告
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::config::AppConfig;
use crate::core::{data_structures::*, SystemEvent};
use crate::storage::DataStore;

pub mod clarifier;
pub mod critic;
pub mod innovator;
pub mod summarizer;
pub mod synthesizer;
pub mod verifier;

/// Agent trait - 所有智能体的基础接口
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn capabilities(&self) -> Vec<AgentCapability>;
    async fn execute(&self, context: AgentContext) -> Result<AgentResult>;
}

/// Agent能力
#[derive(Debug, Clone)]
pub enum AgentCapability {
    Clarification,
    Innovation,
    Criticism,
    Synthesis,
    Verification,
    Summarization,
}

/// Agent执行上下文
#[derive(Debug, Clone)]
pub struct AgentContext {
    pub session_id: uuid::Uuid,
    pub current_version: Option<IterationVersion>,
    pub clarification: Option<Clarification>,
    pub previous_versions: Vec<IterationVersion>,
    pub knowledge_base: Vec<Evidence>,
    pub previous_results: Vec<AgentResult>, // 添加前一个Agent的结果
}

/// Agent执行结果
#[derive(Debug, Clone)]
pub enum AgentResult {
    Clarification(Clarification),
    Innovation(Vec<String>), // Delta list
    Criticism(Vec<Criticism>),
    Synthesis(IterationVersion),
    Verification(VerificationReport),
    Summary(String),
}

/// 批评意见
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Criticism {
    pub delta_index: usize,
    pub category: CriticismCategory,
    pub message: String,
    pub severity: f64, // 0.0 - 1.0
}

/// 批评类别
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CriticismCategory {
    LogicFlaw,
    FeasibilityIssue,
    RiskConcern,
    ValueUnclear,
    Inconsistency,
    ResourceConstraint,
    StakeholderConcern,
    ImplementationGap,
    MarketMismatch,
    ComplexityIssue,
}

/// Agent运行时
pub struct AgentRuntime {
    pub clarifier: Arc<dyn Agent>,
    pub innovator: Arc<dyn Agent>,
    pub critic: Arc<dyn Agent>,
    pub synthesizer: Arc<dyn Agent>,
    pub verifier: Arc<dyn Agent>,
    pub summarizer: Arc<dyn Agent>,
    config: Arc<RwLock<AppConfig>>,
    storage: Arc<DataStore>,
    event_bus: mpsc::UnboundedSender<SystemEvent>,
}

impl AgentRuntime {
    pub async fn new(
        config: Arc<RwLock<AppConfig>>,
        storage: Arc<DataStore>,
        event_bus: mpsc::UnboundedSender<SystemEvent>,
    ) -> Result<Self> {
        // 创建模型管理器
        let model_manager = Arc::new(crate::models::ModelManager::new(config.clone()));

        let clarifier =
            Arc::new(clarifier::ClarifierAgent::new(config.clone(), model_manager.clone()).await?);
        let innovator =
            Arc::new(innovator::InnovatorAgent::new(config.clone(), model_manager.clone()).await?);
        let critic =
            Arc::new(critic::CriticAgent::new(config.clone(), model_manager.clone()).await?);
        let synthesizer = Arc::new(
            synthesizer::SynthesizerAgent::new(config.clone(), model_manager.clone()).await?,
        );
        let verifier = Arc::new(
            verifier::VerifierAgent::new(config.clone(), storage.clone(), model_manager.clone())
                .await?,
        );
        let summarizer = Arc::new(
            summarizer::SummarizerAgent::new(config.clone(), model_manager.clone()).await?,
        );

        Ok(Self {
            clarifier,
            innovator,
            critic,
            synthesizer,
            verifier,
            summarizer,
            config,
            storage,
            event_bus,
        })
    }

    pub async fn run_clarification_round(&self, context: AgentContext) -> Result<Clarification> {
        match self.clarifier.execute(context).await? {
            AgentResult::Clarification(clarification) => Ok(clarification),
            _ => Err(anyhow::anyhow!("Clarifier returned unexpected result type")),
        }
    }

    pub async fn run_adversarial_iteration(
        &self,
        context: AgentContext,
    ) -> Result<IterationVersion> {
        // 1. Innovator generates improvements
        let innovation_result = self.innovator.execute(context.clone()).await?;
        let deltas = match innovation_result {
            AgentResult::Innovation(deltas) => deltas,
            _ => return Err(anyhow::anyhow!("Innovator returned unexpected result type")),
        };

        // 2. Critic reviews the improvements
        let mut critic_context = context.clone();
        critic_context.previous_results = vec![AgentResult::Innovation(deltas.clone())];
        let criticism_result = self.critic.execute(critic_context).await?;
        let criticisms = match criticism_result {
            AgentResult::Criticism(criticisms) => criticisms,
            _ => return Err(anyhow::anyhow!("Critic returned unexpected result type")),
        };

        // 3. Synthesizer merges everything
        let mut synthesis_context = context.clone();
        synthesis_context.previous_results = vec![
            AgentResult::Innovation(deltas),
            AgentResult::Criticism(criticisms),
        ];
        let synthesis_result = self.synthesizer.execute(synthesis_context).await?;
        match synthesis_result {
            AgentResult::Synthesis(version) => Ok(version),
            _ => Err(anyhow::anyhow!(
                "Synthesizer returned unexpected result type"
            )),
        }
    }

    pub async fn run_verification(&self, context: AgentContext) -> Result<VerificationReport> {
        match self.verifier.execute(context).await? {
            AgentResult::Verification(report) => Ok(report),
            _ => Err(anyhow::anyhow!("Verifier returned unexpected result type")),
        }
    }

    pub async fn run_summarization(&self, context: AgentContext) -> Result<String> {
        match self.summarizer.execute(context).await? {
            AgentResult::Summary(summary) => Ok(summary),
            _ => Err(anyhow::anyhow!(
                "Summarizer returned unexpected result type"
            )),
        }
    }
}
