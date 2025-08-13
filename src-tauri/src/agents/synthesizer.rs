use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::agents::{Agent, AgentCapability, AgentContext, AgentResult, Criticism};
use crate::config::AppConfig;
use crate::models::{ModelManager, ChatRequest, ChatMessage};
use crate::core::data_structures::*;

pub struct SynthesizerAgent {
    config: Arc<RwLock<AppConfig>>,
    model_manager: Arc<ModelManager>,
}

impl SynthesizerAgent {
    pub async fn new(config: Arc<RwLock<AppConfig>>, model_manager: Arc<ModelManager>) -> Result<Self> {
        Ok(Self { 
            config,
            model_manager,
        })
    }

    /// 合成改进建议和批评意见，生成新版本（AI驱动）
    async fn synthesize_iteration(
        &self,
        delta_strings: &[String],
        criticisms: &[Criticism],
        current_idea: Option<&StructuredIdea>,
    ) -> Result<IterationVersion> {
        // 构建AI合成请求
        let context = if let Some(idea) = current_idea {
            format!(
                "当前想法背景：\n目标：{}\n受众：{}\n约束：{}\n",
                idea.target.as_deref().unwrap_or("未明确"),
                idea.stakeholders.join(", "),
                idea.constraints.keys().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            )
        } else {
            "没有特定想法背景\n".to_string()
        };

        let deltas_text = delta_strings.iter()
            .enumerate()
            .map(|(i, delta)| format!("{}. {}", i + 1, delta))
            .collect::<Vec<_>>()
            .join("\n");

        let criticisms_text = criticisms.iter()
            .enumerate()
            .map(|(i, criticism)| format!("{}. [严重度:{:.1}] {}", i + 1, criticism.severity, criticism.message))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"你是一个专业的创新合成专家。请基于以下信息合成一个优化版本：

{}

创新建议：
{}

批评意见：
{}

请进行智能合成，并以JSON格式返回：

{{
    "filtered_deltas": ["保留的优质建议1", "保留的优质建议2"],
    "synthesis_reasoning": "合成推理过程，解释为什么选择这些建议，如何应对批评",
    "improvement_summary": "本次优化的核心改进点",
    "confidence_score": 0.85,
    "novelty_score": 0.80,
    "feasibility_score": 0.90,
    "coherence_score": 0.88
}}

合成要求：
1. 过滤掉低质量或高风险的建议
2. 结合批评意见优化建议
3. 确保建议之间的一致性
4. 提供清晰的推理过程
5. 评估合成结果的质量分数"#,
            context, deltas_text, criticisms_text
        );

        let request = ChatRequest {
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            model: "gpt-4".to_string(),
            temperature: Some(0.4),
            max_tokens: Some(1500),
        };

        match self.model_manager.chat(request).await {
            Ok(response) => {
                // 解析AI响应
                if let Ok(synthesis_result) = self.parse_synthesis_response(&response.content) {
                    Ok(synthesis_result)
                } else {
                    // AI解析失败，使用基础合成
                    Ok(self.generate_basic_synthesis(delta_strings, criticisms).await?)
                }
            }
            Err(e) => {
                tracing::warn!("AI合成失败，使用基础合成: {}", e);
                Ok(self.generate_basic_synthesis(delta_strings, criticisms).await?)
            }
        }
    }

    /// 解析AI合成响应
    fn parse_synthesis_response(&self, response: &str) -> Result<IterationVersion> {
        use serde_json::Value;

        let json: Value = serde_json::from_str(response)?;
        
        let filtered_deltas: Vec<String> = json["filtered_deltas"]
            .as_array()
            .map(|arr| arr.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect())
            .unwrap_or_else(|| vec!["基础综合建议".to_string()]);

        let reasoning = json["synthesis_reasoning"]
            .as_str()
            .unwrap_or("基础合成推理")
            .to_string();

        let summary = json["improvement_summary"]
            .as_str()
            .unwrap_or("综合改进版本")
            .to_string();

        let confidence = json["confidence_score"].as_f64().unwrap_or(0.7);
        let novelty = json["novelty_score"].as_f64().unwrap_or(0.7);
        let feasibility = json["feasibility_score"].as_f64().unwrap_or(0.8);
        let coherence = json["coherence_score"].as_f64().unwrap_or(0.8);

        Ok(IterationVersion {
            id: uuid::Uuid::new_v4(),
            version_number: 1, // 会在运行时更新
            summary,
            deltas: filtered_deltas,
            rationale: reasoning,
            scores: Scores {
                novelty,
                feasibility,
                coherence,
            },
            created_at: chrono::Utc::now(),
        })
    }

    /// 生成基础合成结果（AI失败时的后备方案）
    async fn generate_basic_synthesis(
        &self,
        delta_strings: &[String],
        criticisms: &[Criticism],
    ) -> Result<IterationVersion> {
        // 简化的综合逻辑，直接基于字符串处理
        let filtered_deltas = self
            .filter_viable_delta_strings(delta_strings, criticisms)
            .await?;

        // 生成版本信息
        let reasoning = self
            .generate_synthesis_reasoning_simple(&filtered_deltas, criticisms)
            .await?;
        let confidence = self
            .calculate_synthesis_confidence_simple(&filtered_deltas, criticisms)
            .await?;

        Ok(IterationVersion {
            id: uuid::Uuid::new_v4(),
            version_number: 1, // This should be incremented by the runtime
            summary: "综合改进版本".to_string(),
            deltas: filtered_deltas,
            rationale: reasoning,
            scores: Scores {
                novelty: confidence * 0.8,
                feasibility: confidence,
                coherence: confidence * 0.9,
            },
            created_at: chrono::Utc::now(),
        })
    }

    /// 根据批评意见筛选可行的改进建议（字符串版本）
    async fn filter_viable_delta_strings(
        &self,
        delta_strings: &[String],
        criticisms: &[Criticism],
    ) -> Result<Vec<String>> {
        let mut viable_deltas = Vec::new();

        for (index, delta_str) in delta_strings.iter().enumerate() {
            let mut is_viable = true;

            // 检查是否有针对这个delta的严重批评
            for criticism in criticisms {
                if criticism.delta_index == index && criticism.severity > 0.7 {
                    is_viable = false;
                    break;
                }
            }

            if is_viable {
                viable_deltas.push(delta_str.clone());
            }
        }

        Ok(viable_deltas)
    }

    /// 生成综合推理说明（简化版本）
    async fn generate_synthesis_reasoning_simple(
        &self,
        deltas: &[String],
        criticisms: &[Criticism],
    ) -> Result<String> {
        let mut reasoning = String::new();

        reasoning.push_str("## 综合分析\n\n");

        reasoning.push_str(&format!(
            "本次迭代综合了 {} 个改进建议和 {} 个批评意见。\n\n",
            deltas.len(),
            criticisms.len()
        ));

        if !deltas.is_empty() {
            reasoning.push_str("### 采纳的改进建议:\n");
            for (i, delta) in deltas.iter().enumerate() {
                reasoning.push_str(&format!("{}. {}\n", i + 1, delta));
            }
            reasoning.push('\n');
        }

        let critical_criticisms: Vec<_> = criticisms.iter().filter(|c| c.severity > 0.7).collect();

        if !critical_criticisms.is_empty() {
            reasoning.push_str("### 解决的关键问题:\n");
            for (i, criticism) in critical_criticisms.iter().enumerate() {
                reasoning.push_str(&format!("{}. {}\n", i + 1, criticism.message));
            }
            reasoning.push('\n');
        }

        reasoning.push_str("### 综合策略:\n");
        reasoning.push_str("通过平衡创新性和可行性，选择了最具价值且风险可控的改进方案。");

        Ok(reasoning)
    }

    /// 计算综合置信度（简化版本）
    async fn calculate_synthesis_confidence_simple(
        &self,
        deltas: &[String],
        criticisms: &[Criticism],
    ) -> Result<f64> {
        if deltas.is_empty() {
            return Ok(0.1);
        }

        // 基于采纳比例的置信度
        let total_deltas = deltas.len() as f64;
        let adoption_rate = total_deltas / (total_deltas + criticisms.len() as f64);

        // 批评严重程度惩罚
        let critical_count = criticisms.iter().filter(|c| c.severity > 0.7).count();

        let criticism_penalty = if critical_count > 0 {
            1.0 - (critical_count as f64 * 0.2).min(0.5)
        } else {
            1.0
        };

        let confidence = (adoption_rate * criticism_penalty).max(0.1).min(1.0);

        Ok(confidence)
    }
}

#[async_trait::async_trait]
impl Agent for SynthesizerAgent {
    fn name(&self) -> &str {
        "Synthesizer"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![AgentCapability::Synthesis]
    }

    async fn execute(&self, context: AgentContext) -> Result<AgentResult> {
        // 从previous_results中提取改进建议和批评
        let mut deltas = Vec::new();
        let mut criticisms = Vec::new();

        for result in &context.previous_results {
            match result {
                AgentResult::Innovation(innovation_deltas) => {
                    deltas.extend(innovation_deltas.clone());
                }
                AgentResult::Criticism(criticism_list) => {
                    criticisms.extend(criticism_list.clone());
                }
                _ => {} // 忽略其他类型的结果
            }
        }

        if deltas.is_empty() {
            return Err(anyhow::anyhow!("No innovation deltas found in context"));
        }

        // 获取当前版本的想法
        let current_idea = None; // 暂时使用None

        // 执行综合
        let iteration = self
            .synthesize_iteration(&deltas, &criticisms, current_idea)
            .await?;

        Ok(AgentResult::Synthesis(iteration))
    }
}
