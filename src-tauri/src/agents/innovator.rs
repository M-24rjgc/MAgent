use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::agents::{Agent, AgentCapability, AgentContext, AgentResult};
use crate::config::AppConfig;
use crate::models::{ModelManager, ChatRequest, ChatMessage};
use crate::core::data_structures::*;

pub struct InnovatorAgent {
    config: Arc<RwLock<AppConfig>>,
    model_manager: Arc<ModelManager>,
}

/// 创新维度枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InnovationDimension {
    Scope,       // 范围扩展
    Technology,  // 技术创新
    Business,    // 商业模式
    User,        // 用户体验
    Process,     // 流程优化
    Risk,        // 风险管控
    Scale,       // 规模化
    Integration, // 集成整合
}

/// Delta 类型
#[derive(Debug, Clone)]
pub struct Delta {
    pub content: String,
    pub dimension: InnovationDimension,
    pub impact_level: f64,     // 影响程度 0.0-1.0
    pub feasibility: f64,      // 可行性 0.0-1.0
    pub innovation_score: f64, // 创新度 0.0-1.0
    pub reasoning: String,     // 推理过程
}

impl InnovatorAgent {
    pub async fn new(config: Arc<RwLock<AppConfig>>, model_manager: Arc<ModelManager>) -> Result<Self> {
        Ok(Self { 
            config,
            model_manager,
        })
    }

    /// 基于结构化想法生成创新增量
    pub async fn generate_deltas(&self, structured_idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let model = self.model_manager.get_model_for_agent("innovator").await;
        
        let prompt = self.build_innovation_prompt(structured_idea);
        
        let request = ChatRequest {
            model,
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "你是一个创新专家，专门帮助用户从多个维度改进和创新想法。你需要生成具体的、可行的创新建议，并评估其影响度、可行性和创新度。请用中文回答。".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: Some(0.8), // 更高的温度鼓励创新
            max_tokens: Some(3000),
        };

        let response = self.model_manager.chat(request).await?;
        
        // 解析AI响应为Delta列表
        self.parse_ai_response_to_deltas(&response.content, structured_idea).await
    }

    /// 构建创新提示词
    fn build_innovation_prompt(&self, idea: &StructuredIdea) -> String {
        format!(
            r#"请分析以下结构化想法，并从多个维度生成创新改进建议：

**当前想法结构：**
- 目标：{}
- 受众：{}
- 约束条件：{}
- 预期产出：{}
- 成功指标：{}
- 风险假设：{}

**创新任务：**
请从以下6个维度为这个想法生成创新改进建议：

1. **范围扩展 (Scope)**：如何扩大想法的应用范围或覆盖面？
2. **技术创新 (Technology)**：有哪些新技术可以应用来提升效果？
3. **商业模式 (Business)**：有哪些创新的商业模式或盈利方式？
4. **用户体验 (User)**：如何改善用户体验或交互方式？
5. **规模化 (Scale)**：如何实现快速扩张或规模化？
6. **集成整合 (Integration)**：如何与其他系统或平台整合？

**输出格式要求：**
请按以下JSON格式输出，每个维度提供2-3个具体建议：

```json
{{
    "scope_deltas": [
        {{
            "content": "具体的范围扩展建议",
            "impact_level": 0.8,
            "feasibility": 0.7,
            "innovation_score": 0.6,
            "reasoning": "详细的推理过程"
        }}
    ],
    "technology_deltas": [...],
    "business_deltas": [...],
    "user_deltas": [...],
    "scale_deltas": [...],
    "integration_deltas": [...]
}}
```

每个建议需要：
- content: 具体清晰的改进建议
- impact_level: 预期影响程度 (0.0-1.0)
- feasibility: 实施可行性 (0.0-1.0)  
- innovation_score: 创新程度 (0.0-1.0)
- reasoning: 详细说明该建议的价值和实施思路"#,
            idea.target.as_deref().unwrap_or("未明确"),
            idea.stakeholders.join(", "),
            self.format_constraints(&idea.constraints),
            idea.deliverables.join(", "),
            idea.success_metrics.join(", "),
            idea.risks_assumptions.join(", ")
        )
    }

    /// 格式化约束条件
    fn format_constraints(&self, constraints: &std::collections::HashMap<String, String>) -> String {
        if constraints.is_empty() {
            "无特定约束".to_string()
        } else {
            constraints.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    /// 解析AI响应为Delta列表
    async fn parse_ai_response_to_deltas(&self, response: &str, _idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // 尝试解析JSON响应
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response) {
            // 解析各个维度的deltas
            if let Some(scope_array) = parsed.get("scope_deltas").and_then(|v| v.as_array()) {
                for item in scope_array {
                    if let Some(delta) = self.parse_delta_item(item, InnovationDimension::Scope) {
                        deltas.push(delta);
                    }
                }
            }

            if let Some(tech_array) = parsed.get("technology_deltas").and_then(|v| v.as_array()) {
                for item in tech_array {
                    if let Some(delta) = self.parse_delta_item(item, InnovationDimension::Technology) {
                        deltas.push(delta);
                    }
                }
            }

            if let Some(business_array) = parsed.get("business_deltas").and_then(|v| v.as_array()) {
                for item in business_array {
                    if let Some(delta) = self.parse_delta_item(item, InnovationDimension::Business) {
                        deltas.push(delta);
                    }
                }
            }

            if let Some(user_array) = parsed.get("user_deltas").and_then(|v| v.as_array()) {
                for item in user_array {
                    if let Some(delta) = self.parse_delta_item(item, InnovationDimension::User) {
                        deltas.push(delta);
                    }
                }
            }

            if let Some(scale_array) = parsed.get("scale_deltas").and_then(|v| v.as_array()) {
                for item in scale_array {
                    if let Some(delta) = self.parse_delta_item(item, InnovationDimension::Scale) {
                        deltas.push(delta);
                    }
                }
            }

            if let Some(integration_array) = parsed.get("integration_deltas").and_then(|v| v.as_array()) {
                for item in integration_array {
                    if let Some(delta) = self.parse_delta_item(item, InnovationDimension::Integration) {
                        deltas.push(delta);
                    }
                }
            }
        }

        // 如果没有解析到任何delta或解析失败，生成后备deltas
        if deltas.is_empty() {
            deltas = self.generate_fallback_deltas().await;
        }

        // 对Delta进行评分和排序
        let mut ranked_deltas = deltas;
        self.score_and_rank_deltas(&mut ranked_deltas);

        // 返回前5个最佳Delta
        ranked_deltas.truncate(5);
        Ok(ranked_deltas)
    }

    /// 解析单个delta项
    fn parse_delta_item(&self, item: &serde_json::Value, dimension: InnovationDimension) -> Option<Delta> {
        let content = item.get("content")?.as_str()?.to_string();
        let impact_level = item.get("impact_level")?.as_f64().unwrap_or(0.5);
        let feasibility = item.get("feasibility")?.as_f64().unwrap_or(0.5);
        let innovation_score = item.get("innovation_score")?.as_f64().unwrap_or(0.5);
        let reasoning = item.get("reasoning")?.as_str().unwrap_or("").to_string();

        Some(Delta {
            content,
            dimension,
            impact_level,
            feasibility,
            innovation_score,
            reasoning,
        })
    }

    /// 生成后备Delta（当AI解析失败时使用）
    async fn generate_fallback_deltas(&self) -> Vec<Delta> {
        vec![
            Delta {
                content: "考虑将解决方案扩展到相关的垂直领域或行业".to_string(),
                dimension: InnovationDimension::Scope,
                impact_level: 0.7,
                feasibility: 0.6,
                innovation_score: 0.5,
                reasoning: "通过扩大应用范围可以增加影响力".to_string(),
            },
            Delta {
                content: "引入人工智能或机器学习技术来优化核心流程".to_string(),
                dimension: InnovationDimension::Technology,
                impact_level: 0.8,
                feasibility: 0.5,
                innovation_score: 0.8,
                reasoning: "AI技术可以显著提升效率和效果".to_string(),
            },
            Delta {
                content: "设计更加直观和个性化的用户界面".to_string(),
                dimension: InnovationDimension::User,
                impact_level: 0.6,
                feasibility: 0.8,
                innovation_score: 0.4,
                reasoning: "改善用户体验可以提高采用率".to_string(),
            },
        ]
    }

    /// 生成范围扩展Delta
    async fn generate_scope_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        if let Some(target) = &idea.target {
            // 纵向扩展
            deltas.push(Delta {
                content: format!(
                    "将「{}」的目标从单点突破扩展到全链条覆盖，形成端到端解决方案",
                    target
                ),
                dimension: InnovationDimension::Scope,
                impact_level: 0.8,
                feasibility: 0.6,
                innovation_score: 0.7,
                reasoning: "通过全链条思维可以发现更多价值点和协同效应".to_string(),
            });

            // 横向扩展
            deltas.push(Delta {
                content: format!("基于「{}」的核心能力，拓展到相邻领域和场景应用", target),
                dimension: InnovationDimension::Scope,
                impact_level: 0.7,
                feasibility: 0.7,
                innovation_score: 0.6,
                reasoning: "核心能力的横向复用可以快速扩大影响范围".to_string(),
            });
        }

        // 利益相关者扩展
        if !idea.stakeholders.is_empty() {
            deltas.push(Delta {
                content: "识别和纳入间接利益相关者，构建更广泛的生态协作网络".to_string(),
                dimension: InnovationDimension::Scope,
                impact_level: 0.6,
                feasibility: 0.8,
                innovation_score: 0.5,
                reasoning: "生态思维可以创造更大的整体价值和可持续性".to_string(),
            });
        }

        Ok(deltas)
    }

    /// 生成技术创新Delta
    async fn generate_technology_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // AI/自动化增强
        deltas.push(Delta {
            content: "引入AI和自动化技术，提升效率并实现智能化升级".to_string(),
            dimension: InnovationDimension::Technology,
            impact_level: 0.9,
            feasibility: 0.7,
            innovation_score: 0.8,
            reasoning: "AI技术可以显著提升处理能力和用户体验".to_string(),
        });

        // 数据驱动优化
        deltas.push(Delta {
            content: "建立数据收集和分析体系，实现基于数据的持续优化".to_string(),
            dimension: InnovationDimension::Technology,
            impact_level: 0.7,
            feasibility: 0.8,
            innovation_score: 0.6,
            reasoning: "数据驱动可以实现精准决策和个性化服务".to_string(),
        });

        // 平台化架构
        if !idea.deliverables.is_empty() {
            deltas.push(Delta {
                content: "采用平台化架构设计，支持模块化扩展和第三方集成".to_string(),
                dimension: InnovationDimension::Technology,
                impact_level: 0.8,
                feasibility: 0.6,
                innovation_score: 0.7,
                reasoning: "平台化可以提高复用性和可扩展性".to_string(),
            });
        }

        Ok(deltas)
    }

    /// 生成商业模式创新Delta
    async fn generate_business_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // 价值网络重构
        deltas.push(Delta {
            content: "重新设计价值创造和分配机制，构建多方共赢的商业生态".to_string(),
            dimension: InnovationDimension::Business,
            impact_level: 0.8,
            feasibility: 0.5,
            innovation_score: 0.9,
            reasoning: "创新的价值分配机制可以激发更大的参与动力".to_string(),
        });

        // 订阅/服务化模式
        deltas.push(Delta {
            content: "从一次性交付转向持续服务模式，建立长期客户关系".to_string(),
            dimension: InnovationDimension::Business,
            impact_level: 0.7,
            feasibility: 0.7,
            innovation_score: 0.6,
            reasoning: "服务化模式可以提供更稳定的收入和更深的客户绑定".to_string(),
        });

        // 免费增值模式
        if !idea.stakeholders.is_empty() {
            deltas.push(Delta {
                content: "采用免费增值策略，通过基础功能免费快速扩大用户基础".to_string(),
                dimension: InnovationDimension::Business,
                impact_level: 0.6,
                feasibility: 0.8,
                innovation_score: 0.5,
                reasoning: "免费策略可以快速获取用户，后续通过增值服务变现".to_string(),
            });
        }

        Ok(deltas)
    }

    /// 生成用户体验创新Delta
    async fn generate_user_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // 个性化体验
        deltas.push(Delta {
            content: "基于用户行为和偏好数据，提供个性化的体验和推荐".to_string(),
            dimension: InnovationDimension::User,
            impact_level: 0.8,
            feasibility: 0.7,
            innovation_score: 0.7,
            reasoning: "个性化可以显著提升用户满意度和粘性".to_string(),
        });

        // 多模态交互
        deltas.push(Delta {
            content: "支持语音、手势、AR/VR等多种交互方式，提升交互自然性".to_string(),
            dimension: InnovationDimension::User,
            impact_level: 0.7,
            feasibility: 0.6,
            innovation_score: 0.8,
            reasoning: "多模态交互可以降低使用门槛并提升用户体验".to_string(),
        });

        // 社区化功能
        if idea.stakeholders.len() > 1 {
            deltas.push(Delta {
                content: "构建用户社区和协作功能，促进用户间的知识分享和互助".to_string(),
                dimension: InnovationDimension::User,
                impact_level: 0.6,
                feasibility: 0.8,
                innovation_score: 0.6,
                reasoning: "社区化可以增强用户粘性和价值创造".to_string(),
            });
        }

        Ok(deltas)
    }

    /// 生成流程优化Delta
    async fn generate_process_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // 敏捷迭代
        deltas.push(Delta {
            content: "采用敏捷开发和快速迭代模式，加快响应速度和适应性".to_string(),
            dimension: InnovationDimension::Process,
            impact_level: 0.7,
            feasibility: 0.8,
            innovation_score: 0.5,
            reasoning: "敏捷模式可以快速验证想法并降低风险".to_string(),
        });

        // 自动化工作流
        deltas.push(Delta {
            content: "设计自动化工作流程，减少人工干预和提高处理效率".to_string(),
            dimension: InnovationDimension::Process,
            impact_level: 0.8,
            feasibility: 0.7,
            innovation_score: 0.6,
            reasoning: "自动化可以显著提升效率和一致性".to_string(),
        });

        Ok(deltas)
    }

    /// 生成风险管控Delta
    async fn generate_risk_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // 分阶段验证
        deltas.push(Delta {
            content: "设计分阶段验证机制，通过MVP和原型快速验证核心假设".to_string(),
            dimension: InnovationDimension::Risk,
            impact_level: 0.8,
            feasibility: 0.9,
            innovation_score: 0.5,
            reasoning: "分阶段验证可以最小化风险和资源浪费".to_string(),
        });

        // 备选方案
        deltas.push(Delta {
            content: "为关键环节准备备选技术方案和实施路径".to_string(),
            dimension: InnovationDimension::Risk,
            impact_level: 0.6,
            feasibility: 0.8,
            innovation_score: 0.4,
            reasoning: "备选方案可以降低单点失败风险".to_string(),
        });

        // 风险监控
        if !idea.risks_assumptions.is_empty() {
            deltas.push(Delta {
                content: "建立实时风险监控和预警机制，及时识别和应对风险".to_string(),
                dimension: InnovationDimension::Risk,
                impact_level: 0.7,
                feasibility: 0.7,
                innovation_score: 0.6,
                reasoning: "主动风险管理可以避免问题扩大化".to_string(),
            });
        }

        Ok(deltas)
    }

    /// 生成规模化Delta
    async fn generate_scale_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // 云原生架构
        deltas.push(Delta {
            content: "采用云原生架构和微服务设计，支持弹性伸缩和高可用".to_string(),
            dimension: InnovationDimension::Scale,
            impact_level: 0.8,
            feasibility: 0.7,
            innovation_score: 0.6,
            reasoning: "云原生架构为未来扩展提供技术基础".to_string(),
        });

        // 标准化复制
        deltas.push(Delta {
            content: "建立标准化的部署和运营模式，支持快速复制到新市场".to_string(),
            dimension: InnovationDimension::Scale,
            impact_level: 0.7,
            feasibility: 0.8,
            innovation_score: 0.5,
            reasoning: "标准化可以降低扩展成本和复杂度".to_string(),
        });

        Ok(deltas)
    }

    /// 生成集成整合Delta
    async fn generate_integration_deltas(&self, idea: &StructuredIdea) -> Result<Vec<Delta>> {
        let mut deltas = Vec::new();

        // API生态
        deltas.push(Delta {
            content: "构建开放API生态，支持第三方集成和合作伙伴扩展".to_string(),
            dimension: InnovationDimension::Integration,
            impact_level: 0.7,
            feasibility: 0.8,
            innovation_score: 0.7,
            reasoning: "开放生态可以借助外部力量加速发展".to_string(),
        });

        // 数据互通
        deltas.push(Delta {
            content: "设计数据互通和同步机制，与现有系统无缝集成".to_string(),
            dimension: InnovationDimension::Integration,
            impact_level: 0.8,
            feasibility: 0.6,
            innovation_score: 0.6,
            reasoning: "数据互通可以减少使用门槛和切换成本".to_string(),
        });

        Ok(deltas)
    }

    /// 对Delta进行评分和排序
    fn score_and_rank_deltas(&self, deltas: &mut Vec<Delta>) {
        // 计算综合分数：影响程度 * 0.4 + 可行性 * 0.3 + 创新度 * 0.3
        for delta in deltas.iter_mut() {
            let composite_score =
                delta.impact_level * 0.4 + delta.feasibility * 0.3 + delta.innovation_score * 0.3;
            delta.impact_level = composite_score; // 复用字段存储综合分数
        }

        // 按综合分数降序排序
        deltas.sort_by(|a, b| b.impact_level.partial_cmp(&a.impact_level).unwrap());
    }

    /// 生成Delta总结报告
    fn generate_delta_summary(&self, deltas: &[Delta]) -> String {
        let mut summary = "🚀 创新增量分析报告\n\n".to_string();

        // 按维度统计
        let mut dimension_counts = std::collections::HashMap::new();
        for delta in deltas {
            *dimension_counts.entry(&delta.dimension).or_insert(0) += 1;
        }

        summary.push_str("📊 创新维度分布：\n");
        for (dim, count) in dimension_counts {
            let dim_name = match dim {
                InnovationDimension::Scope => "范围扩展",
                InnovationDimension::Technology => "技术创新",
                InnovationDimension::Business => "商业模式",
                InnovationDimension::User => "用户体验",
                InnovationDimension::Process => "流程优化",
                InnovationDimension::Risk => "风险管控",
                InnovationDimension::Scale => "规模化",
                InnovationDimension::Integration => "集成整合",
            };
            summary.push_str(&format!("• {}: {} 个建议\n", dim_name, count));
        }

        summary.push_str("\n🎯 重点推荐的创新方向：\n");
        for (i, delta) in deltas.iter().take(3).enumerate() {
            summary.push_str(&format!(
                "{}. {}\n   💡 {}\n\n",
                i + 1,
                delta.content,
                delta.reasoning
            ));
        }

        summary
    }
}

#[async_trait]
impl Agent for InnovatorAgent {
    fn name(&self) -> &str {
        "Innovator"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![AgentCapability::Innovation]
    }

    async fn execute(&self, context: AgentContext) -> Result<AgentResult> {
        tracing::info!("Innovator executing for session: {}", context.session_id);

        // 检查是否有澄清结果作为输入
        let structured_idea = if let Some(clarification) = &context.clarification {
            clarification.structured_idea.clone()
        } else {
            None
        };

        let delta_strings = if let Some(idea) = structured_idea {
            // 基于结构化想法生成创新Delta
            let deltas = self.generate_deltas(&idea).await?;

            // 记录生成的Delta信息
            tracing::info!("Generated {} innovation deltas", deltas.len());
            for delta in &deltas {
                tracing::debug!(
                    "Delta [{}]: {} (impact: {:.2})",
                    format!("{:?}", delta.dimension),
                    delta.content,
                    delta.impact_level
                );
            }

            // 生成总结报告
            let summary = self.generate_delta_summary(&deltas);
            tracing::info!("Innovation summary: {}", summary);

            // 转换为字符串列表
            deltas
                .into_iter()
                .map(|d| {
                    format!(
                        "[{}] {}",
                        match d.dimension {
                            InnovationDimension::Scope => "范围扩展",
                            InnovationDimension::Technology => "技术创新",
                            InnovationDimension::Business => "商业模式",
                            InnovationDimension::User => "用户体验",
                            InnovationDimension::Process => "流程优化",
                            InnovationDimension::Risk => "风险管控",
                            InnovationDimension::Scale => "规模化",
                            InnovationDimension::Integration => "集成整合",
                        },
                        d.content
                    )
                })
                .collect()
        } else {
            // 如果没有结构化想法，返回通用创新建议
            tracing::warn!(
                "No structured idea provided, generating generic innovation suggestions"
            );
            vec![
                "[技术创新] 引入AI和自动化技术提升效率".to_string(),
                "[用户体验] 设计更直观友好的用户界面".to_string(),
                "[商业模式] 探索订阅制或平台化商业模式".to_string(),
                "[范围扩展] 考虑横向或纵向扩展应用领域".to_string(),
                "[风险管控] 建立分阶段验证和风险监控机制".to_string(),
            ]
        };

        tracing::info!(
            "Innovator completed with {} delta suggestions",
            delta_strings.len()
        );
        Ok(AgentResult::Innovation(delta_strings))
    }
}
