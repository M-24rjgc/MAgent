// 抑制开发期间的未使用代码警告
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub engine: EngineConfig,
    pub models: ModelRegistryConfig,
    pub retrieval: RetrievalConfig,
    pub performance: PerformanceConfig,
    pub ui: UIConfig,
    pub api_keys: ApiKeysConfig,
}

/// API 密钥配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeysConfig {
    pub openai_api_key: Option<String>,
    pub openai_base_url: Option<String>,
    pub claude_api_key: Option<String>,
    pub deepseek_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
}

/// 引擎配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub clarify: ClarifyConfig,
    pub iteration: IterationConfig,
    pub verification: VerificationConfig,
}

/// 澄清配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClarifyConfig {
    pub max_rounds: u32,
    pub confidence_threshold: f64,
    pub questions_per_round: u32,
}

/// 迭代配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationConfig {
    pub max_iterations: u32,
    pub improvement_threshold: f64,
    pub min_consecutive_improvements: u32,
    pub innovation_styles: Vec<String>,
}

/// 验证配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    pub enable_fact_checking: bool,
    pub enable_logic_checking: bool,
    pub confidence_threshold: f64,
    pub max_evidence_sources: u32,
}

/// 模型注册配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRegistryConfig {
    pub clarifier: String,
    pub innovator: String,
    pub critic: String,
    pub synthesizer: String,
    pub verifier: String,
    pub summarizer: String,
    pub embedding: String,
    pub fallback_chains: HashMap<String, Vec<String>>,
}

/// 检索配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalConfig {
    pub embed_model: String,
    pub chunk_size: usize,
    pub overlap: usize,
    pub max_results: usize,
    pub relevance_threshold: f64,
}

/// 性能配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub max_concurrent_agents: u32,
    pub timeout_seconds: u64,
    pub cache_size: usize,
    pub enable_streaming: bool,
}

/// UI配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub theme: String,
    pub language: String,
    pub auto_save: bool,
    pub show_debug_info: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            engine: EngineConfig {
                clarify: ClarifyConfig {
                    max_rounds: 5,
                    confidence_threshold: 0.8,
                    questions_per_round: 3,
                },
                iteration: IterationConfig {
                    max_iterations: 10,
                    improvement_threshold: 0.01,
                    min_consecutive_improvements: 3,
                    innovation_styles: vec![
                        "conservative".to_string(),
                        "aggressive".to_string(),
                        "practical".to_string(),
                    ],
                },
                verification: VerificationConfig {
                    enable_fact_checking: true,
                    enable_logic_checking: true,
                    confidence_threshold: 0.7,
                    max_evidence_sources: 5,
                },
            },
            models: ModelRegistryConfig {
                clarifier: "gpt-4o-mini".to_string(),
                innovator: "claude-3-5-sonnet-20241022".to_string(),
                critic: "gpt-4o".to_string(),
                synthesizer: "claude-3-5-sonnet-20241022".to_string(),
                verifier: "gpt-4o".to_string(),
                summarizer: "gpt-4o-mini".to_string(),
                embedding: "text-embedding-3-large".to_string(),
                fallback_chains: {
                    let mut chains = HashMap::new();
                    chains.insert("gpt-4o".to_string(), vec!["gpt-4o-mini".to_string(), "claude-3-5-sonnet-20241022".to_string()]);
                    chains.insert("claude-3-5-sonnet-20241022".to_string(), vec!["gpt-4o".to_string(), "gpt-4o-mini".to_string()]);
                    chains
                },
            },
            retrieval: RetrievalConfig {
                embed_model: "text-embedding-3-large".to_string(),
                chunk_size: 1000,
                overlap: 150,
                max_results: 20,
                relevance_threshold: 0.7,
            },
            performance: PerformanceConfig {
                max_concurrent_agents: 5,
                timeout_seconds: 30,
                cache_size: 1000,
                enable_streaming: true,
            },
            ui: UIConfig {
                theme: "light".to_string(),
                language: "zh-CN".to_string(),
                auto_save: true,
                show_debug_info: false,
            },
            api_keys: ApiKeysConfig {
                openai_api_key: None,
                openai_base_url: Some("https://api.openai.com/v1".to_string()),
                claude_api_key: None,
                deepseek_api_key: None,
                gemini_api_key: None,
            },
        }
    }
}

impl AppConfig {
    /// 创建一个新的应用配置实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 便捷访问API密钥的方法
    pub fn openai_api_key(&self) -> Option<&str> {
        self.api_keys.openai_api_key.as_deref()
    }
    
    pub fn openai_base_url(&self) -> Option<&str> {
        self.api_keys.openai_base_url.as_deref()
    }
    
    pub fn claude_api_key(&self) -> Option<&str> {
        self.api_keys.claude_api_key.as_deref()
    }
    
    pub fn deepseek_api_key(&self) -> Option<&str> {
        self.api_keys.deepseek_api_key.as_deref()
    }
    
    pub fn gemini_api_key(&self) -> Option<&str> {
        self.api_keys.gemini_api_key.as_deref()
    }
}
