use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 用户输入的原始想法种子
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaSeed {
    pub raw_text: String,
    pub context_hints: Vec<String>,
    pub domain: Option<String>,
}

/// 澄清阶段的问答对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAPair {
    pub question: String,
    pub answer: Option<String>,
    pub slot_type: SlotType,
}

/// 槽位类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SlotType {
    Target,          // 目标
    Stakeholder,     // 受众/利益相关者
    Constraints,     // 约束条件
    Deliverable,     // 产出形式
    Metrics,         // 价值指标
    RiskAssumptions, // 风险假设
}

/// 澄清结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clarification {
    pub qa_pairs: Vec<QAPair>,
    pub open_slots: Vec<SlotType>,
    pub confidence: f64,
    pub structured_idea: Option<StructuredIdea>,
}

/// 结构化想法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredIdea {
    pub target: Option<String>,
    pub stakeholders: Vec<String>,
    pub constraints: HashMap<String, String>,
    pub deliverables: Vec<String>,
    pub success_metrics: Vec<String>,
    pub risks_assumptions: Vec<String>,
}

/// 迭代版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationVersion {
    pub id: Uuid,
    pub version_number: u32,
    pub summary: String,
    pub deltas: Vec<String>,
    pub rationale: String,
    pub scores: Scores,
    pub created_at: DateTime<Utc>,
}

/// 评分指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scores {
    pub novelty: f64,
    pub feasibility: f64,
    pub coherence: f64,
}

/// 验证报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationReport {
    pub logic_checks: Vec<LogicCheck>,
    pub fact_checks: Vec<FactCheck>,
    pub risks: Vec<Risk>,
    pub passed: bool,
    pub confidence: f64,
}

/// 逻辑检查
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicCheck {
    pub check_type: String,
    pub description: String,
    pub passed: bool,
    pub message: String,
}

/// 事实检查
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactCheck {
    pub claim: String,
    pub evidence: Vec<Evidence>,
    pub status: FactCheckStatus,
    pub confidence: f64,
}

/// 事实检查状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactCheckStatus {
    Supported,
    Partial,
    Unsupported,
    NeedClarification,
}

/// 证据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub source_id: String,
    pub snippet: String,
    pub relevance: f64,
    pub url: Option<String>,
}

/// 风险
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub description: String,
    pub severity: RiskSeverity,
    pub mitigation: Option<String>,
}

/// 风险严重程度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// 输出规范
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSpec {
    pub format: OutputFormat,
    pub sections: Vec<SectionSpec>,
    pub visuals: Vec<VisualSpec>,
}

/// 输出格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    Json,
    Html,
    Pdf,
}

/// 章节规范
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionSpec {
    pub title: String,
    pub content_type: String,
    pub required: bool,
}

/// 可视化规范
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualSpec {
    pub chart_type: String,
    pub data_source: String,
    pub title: String,
}
