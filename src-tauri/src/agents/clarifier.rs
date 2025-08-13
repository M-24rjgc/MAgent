// 抑制开发期间的未使用代码警告
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::agents::{Agent, AgentCapability, AgentContext, AgentResult};
use crate::config::AppConfig;
use crate::core::data_structures::*;
use crate::models::{ChatMessage, ChatRequest, ModelManager};

pub struct ClarifierAgent {
    config: Arc<RwLock<AppConfig>>,
    model_manager: Arc<ModelManager>,
}

impl ClarifierAgent {
    pub async fn new(
        config: Arc<RwLock<AppConfig>>,
        model_manager: Arc<ModelManager>,
    ) -> Result<Self> {
        Ok(Self {
            config,
            model_manager,
        })
    }

    /// 使用AI分析想法并生成澄清问题
    pub async fn analyze_and_clarify(&self, idea: &IdeaSeed) -> Result<Clarification> {
        // 尝试使用AI分析
        match self.try_ai_clarification(idea).await {
            Ok(clarification) => Ok(clarification),
            Err(e) => {
                eprintln!("AI clarification failed, using fallback: {}", e);
                self.generate_fallback_clarification(idea).await
            }
        }
    }

    /// 尝试AI澄清分析
    async fn try_ai_clarification(&self, idea: &IdeaSeed) -> Result<Clarification> {
        let model = self.model_manager.get_model_for_agent("clarifier").await;

        let prompt = self.build_clarification_prompt(idea);

        let request = ChatRequest {
            model,
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "你是一个专业的想法澄清专家。你的任务是分析用户的想法，识别不清楚的部分，并生成针对性的问题来帮助完善这个想法。请用中文回答。".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: Some(0.3),
            max_tokens: Some(2000),
        };

        let response = self.model_manager.chat(request).await?;

        // 解析AI响应并构建Clarification
        self.parse_ai_response_to_clarification(&response.content, idea)
            .await
    }

    /// 生成回退澄清（当AI调用失败时使用）
    async fn generate_fallback_clarification(&self, idea: &IdeaSeed) -> Result<Clarification> {
        // 基于规则的分析，生成合理的澄清问题
        let missing_slots = vec![
            SlotType::Target,
            SlotType::Stakeholder,
            SlotType::Constraints,
            SlotType::Deliverable,
        ];

        let qa_pairs = vec![
            QAPair {
                question: "这个想法的具体目标是什么？希望解决什么问题？".to_string(),
                answer: None,
                slot_type: SlotType::Target,
            },
            QAPair {
                question: "主要的目标用户或受众是谁？".to_string(),
                answer: None,
                slot_type: SlotType::Stakeholder,
            },
            QAPair {
                question: "在实施过程中可能面临哪些限制条件？".to_string(),
                answer: None,
                slot_type: SlotType::Constraints,
            },
            QAPair {
                question: "期望的最终产出或成果是什么？".to_string(),
                answer: None,
                slot_type: SlotType::Deliverable,
            },
        ];

        Ok(Clarification {
            qa_pairs,
            open_slots: missing_slots,
            confidence: 0.7,
            structured_idea: None,
        })
    }

    /// 构建澄清提示词
    fn build_clarification_prompt(&self, idea: &IdeaSeed) -> String {
        format!(
            r#"请分析以下想法，并识别需要澄清的关键信息：

**原始想法：**
{}

**澄清分析要求：**
1. 识别缺失的关键信息槽位（目标、受众、约束条件、期望产出、成功指标、风险假设）
2. 为每个缺失的槽位生成2-3个具体的澄清问题
3. 评估当前想法的清晰度（0-1分）
4. 提供改进建议

**请按以下JSON格式回答：**
```json
{{
    "missing_slots": ["target", "stakeholder", "constraints"],
    "questions": [
        {{
            "question": "具体问题内容",
            "slot": "target"
        }}
    ],
    "clarity_score": 0.6
}}
```"#,
            idea.raw_text
        )
    }

    /// 解析AI响应为Clarification结构
    async fn parse_ai_response_to_clarification(
        &self,
        response: &str,
        idea: &IdeaSeed,
    ) -> Result<Clarification> {
        // 尝试解析JSON响应
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(response) {
            let clarity_score = parsed
                .get("clarity_score")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.5);

            let missing_slots =
                if let Some(slots_array) = parsed.get("missing_slots").and_then(|v| v.as_array()) {
                    slots_array
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| self.string_to_slot_type(s))
                        .collect()
                } else {
                    self.extract_missing_slots(idea).await
                };

            let qa_pairs =
                if let Some(questions_array) = parsed.get("questions").and_then(|v| v.as_array()) {
                    questions_array
                        .iter()
                        .filter_map(|q| {
                            let question_text = q.get("question")?.as_str()?;
                            let slot_str = q.get("slot")?.as_str()?;

                            Some(QAPair {
                                question: question_text.to_string(),
                                answer: None,
                                slot_type: self.string_to_slot_type(slot_str),
                            })
                        })
                        .collect()
                } else {
                    self.generate_fallback_questions(&missing_slots)
                };

            Ok(Clarification {
                qa_pairs,
                open_slots: missing_slots,
                confidence: clarity_score,
                structured_idea: None,
            })
        } else {
            // 如果JSON解析失败，使用传统方法
            let missing_slots = self.extract_missing_slots(idea).await;
            let qa_pairs = self.generate_fallback_questions(&missing_slots);

            Ok(Clarification {
                qa_pairs,
                open_slots: missing_slots,
                confidence: 0.6,
                structured_idea: None,
            })
        }
    }

    /// 字符串转换为SlotType
    fn string_to_slot_type(&self, s: &str) -> SlotType {
        match s.to_lowercase().as_str() {
            "target" | "目标" => SlotType::Target,
            "stakeholder" | "受众" => SlotType::Stakeholder,
            "constraints" | "约束" => SlotType::Constraints,
            "deliverable" | "产出" => SlotType::Deliverable,
            "metrics" | "指标" => SlotType::Metrics,
            "risks" | "风险" => SlotType::RiskAssumptions,
            _ => SlotType::Target, // 默认
        }
    }

    /// 生成后备问题（当AI解析失败时使用）
    fn generate_fallback_questions(&self, missing_slots: &[SlotType]) -> Vec<QAPair> {
        missing_slots
            .iter()
            .map(|slot| {
                let question = match slot {
                    SlotType::Target => "您希望通过这个想法实现什么具体目标？",
                    SlotType::Stakeholder => "谁是这个想法的主要受众或利益相关者？",
                    SlotType::Constraints => "您在实现这个想法时面临哪些限制或约束条件？",
                    SlotType::Deliverable => "您期望的最终产出或交付物是什么形式？",
                    SlotType::Metrics => "您如何衡量这个想法的成功？有哪些关键指标？",
                    SlotType::RiskAssumptions => "您认为在实现过程中可能遇到哪些风险？",
                };

                QAPair {
                    question: question.to_string(),
                    answer: None,
                    slot_type: slot.clone(),
                }
            })
            .collect()
    }
    async fn extract_missing_slots(&self, idea: &IdeaSeed) -> Vec<SlotType> {
        let content = idea.raw_text.to_lowercase();
        let mut missing_slots = Vec::new();

        // 目标(Target)槽位识别
        if !self.has_clear_target(&content) {
            missing_slots.push(SlotType::Target);
        }

        // 受众(Stakeholder)槽位提取
        if !self.has_clear_stakeholders(&content) {
            missing_slots.push(SlotType::Stakeholder);
        }

        // 约束(Constraints)槽位解析
        if !self.has_clear_constraints(&content) {
            missing_slots.push(SlotType::Constraints);
        }

        // 产出形式(Deliverable)槽位分析
        if !self.has_clear_deliverable(&content) {
            missing_slots.push(SlotType::Deliverable);
        }

        // 价值指标(Metrics)槽位定义
        if !self.has_clear_metrics(&content) {
            missing_slots.push(SlotType::Metrics);
        }

        // 风险假设(RiskAssumptions)槽位
        if !self.has_clear_risks(&content) {
            missing_slots.push(SlotType::RiskAssumptions);
        }

        missing_slots
    }

    fn has_clear_target(&self, content: &str) -> bool {
        let target_keywords = [
            "目标",
            "目的",
            "为了",
            "实现",
            "达到",
            "完成",
            "希望",
            "goal",
            "objective",
            "aim",
            "target",
            "achieve",
            "want",
            "need",
        ];
        target_keywords
            .iter()
            .any(|&keyword| content.contains(keyword))
    }

    fn has_clear_stakeholders(&self, content: &str) -> bool {
        let stakeholder_keywords = [
            "用户",
            "客户",
            "团队",
            "公司",
            "组织",
            "受众",
            "人员",
            "群体",
            "user",
            "customer",
            "team",
            "company",
            "stakeholder",
            "audience",
            "people",
        ];
        stakeholder_keywords
            .iter()
            .any(|&keyword| content.contains(keyword))
    }

    fn has_clear_constraints(&self, content: &str) -> bool {
        let constraint_keywords = [
            "预算",
            "时间",
            "资源",
            "限制",
            "约束",
            "要求",
            "条件",
            "成本",
            "budget",
            "time",
            "resource",
            "limit",
            "constraint",
            "requirement",
            "cost",
        ];
        constraint_keywords
            .iter()
            .any(|&keyword| content.contains(keyword))
    }

    fn has_clear_deliverable(&self, content: &str) -> bool {
        let deliverable_keywords = [
            "产品",
            "系统",
            "方案",
            "报告",
            "文档",
            "应用",
            "平台",
            "工具",
            "product",
            "system",
            "solution",
            "report",
            "document",
            "application",
            "platform",
        ];
        deliverable_keywords
            .iter()
            .any(|&keyword| content.contains(keyword))
    }

    fn has_clear_metrics(&self, content: &str) -> bool {
        let metrics_keywords = [
            "指标",
            "效果",
            "收益",
            "价值",
            "成功",
            "kpi",
            "衡量",
            "评估",
            "metric",
            "success",
            "value",
            "benefit",
            "roi",
            "performance",
            "measure",
        ];
        metrics_keywords
            .iter()
            .any(|&keyword| content.contains(keyword))
    }

    fn has_clear_risks(&self, content: &str) -> bool {
        let risk_keywords = [
            "风险",
            "挑战",
            "问题",
            "困难",
            "阻碍",
            "假设",
            "不确定",
            "risk",
            "challenge",
            "problem",
            "difficulty",
            "assumption",
            "uncertainty",
        ];
        risk_keywords
            .iter()
            .any(|&keyword| content.contains(keyword))
    }

    /// 智能问题生成 - 基于缺失槽位生成有针对性的问题
    async fn generate_questions(
        &self,
        idea: &IdeaSeed,
        current_clarification: &Clarification,
    ) -> Result<Vec<QAPair>> {
        let mut questions = Vec::new();

        // 基于open_slots生成问题，并考虑原始想法的内容
        for slot in &current_clarification.open_slots {
            let slot_questions = self.generate_slot_specific_questions(slot, idea).await?;
            questions.extend(slot_questions);
        }

        // 对问题进行优先级排序和去重
        self.prioritize_and_filter_questions(&mut questions);

        // 限制问题数量，避免认知过载
        questions.truncate(3);

        Ok(questions)
    }

    async fn generate_slot_specific_questions(
        &self,
        slot: &SlotType,
        idea: &IdeaSeed,
    ) -> Result<Vec<QAPair>> {
        let mut questions = Vec::new();
        let content = &idea.raw_text;

        match slot {
            SlotType::Target => {
                questions.push(QAPair {
                    question: format!(
                        "基于您提到的「{}」，您希望具体达成什么目标？请尽可能详细地描述。",
                        self.extract_key_phrase(content)
                    ),
                    answer: None,
                    slot_type: SlotType::Target,
                });
                questions.push(QAPair {
                    question: "这个想法要解决的核心问题是什么？为什么这个问题值得解决？"
                        .to_string(),
                    answer: None,
                    slot_type: SlotType::Target,
                });
            }
            SlotType::Stakeholder => {
                questions.push(QAPair {
                    question: "谁是这个想法的主要受益者？他们目前面临什么痛点？".to_string(),
                    answer: None,
                    slot_type: SlotType::Stakeholder,
                });
                questions.push(QAPair {
                    question: "需要哪些团队或个人参与实施？各自的角色和责任是什么？".to_string(),
                    answer: None,
                    slot_type: SlotType::Stakeholder,
                });
            }
            SlotType::Constraints => {
                questions.push(QAPair {
                    question: "您有什么预算、时间或人员方面的限制？这些限制如何影响方案设计？"
                        .to_string(),
                    answer: None,
                    slot_type: SlotType::Constraints,
                });
                questions.push(QAPair {
                    question: "有哪些技术、法律、政策或其他方面的约束需要考虑？".to_string(),
                    answer: None,
                    slot_type: SlotType::Constraints,
                });
            }
            SlotType::Deliverable => {
                questions.push(QAPair {
                    question: "您期望的最终交付物是什么？（如产品、系统、报告、方案等）"
                        .to_string(),
                    answer: None,
                    slot_type: SlotType::Deliverable,
                });
                questions.push(QAPair {
                    question: "这个交付物需要具备哪些关键特性或功能？用户如何使用它？".to_string(),
                    answer: None,
                    slot_type: SlotType::Deliverable,
                });
            }
            SlotType::Metrics => {
                questions.push(QAPair {
                    question: "您如何定义和衡量这个想法的成功？有什么具体的量化指标吗？"
                        .to_string(),
                    answer: None,
                    slot_type: SlotType::Metrics,
                });
                questions.push(QAPair {
                    question: "预期的投入产出比是什么？多长时间能看到效果？".to_string(),
                    answer: None,
                    slot_type: SlotType::Metrics,
                });
            }
            SlotType::RiskAssumptions => {
                questions.push(QAPair {
                    question: "实施过程中可能遇到哪些主要风险或挑战？".to_string(),
                    answer: None,
                    slot_type: SlotType::RiskAssumptions,
                });
                questions.push(QAPair {
                    question: "这个想法基于哪些关键假设？如果假设不成立会怎样？".to_string(),
                    answer: None,
                    slot_type: SlotType::RiskAssumptions,
                });
            }
        }

        Ok(questions)
    }

    fn extract_key_phrase(&self, content: &str) -> String {
        // 简单的关键短语提取逻辑
        let words: Vec<&str> = content.split_whitespace().collect();
        if words.len() > 5 {
            words[..5].join(" ") + "..."
        } else {
            content.to_string()
        }
    }

    fn prioritize_and_filter_questions(&self, questions: &mut Vec<QAPair>) {
        // 去重：移除相似的问题
        questions.dedup_by(|a, b| {
            a.question.chars().take(20).collect::<String>()
                == b.question.chars().take(20).collect::<String>()
        });

        // 简单排序：目标和利益相关者优先
        questions.sort_by(|a, b| {
            let priority_a = match a.slot_type {
                SlotType::Target => 3,
                SlotType::Stakeholder => 2,
                SlotType::Deliverable => 1,
                _ => 0,
            };
            let priority_b = match b.slot_type {
                SlotType::Target => 3,
                SlotType::Stakeholder => 2,
                SlotType::Deliverable => 1,
                _ => 0,
            };
            priority_b.cmp(&priority_a)
        });
    }

    async fn analyze_slot_completeness(&self, clarification: &Clarification) -> f64 {
        let total_slots = 6; // 总共6种槽位类型
        let filled_slots = total_slots - clarification.open_slots.len();
        filled_slots as f64 / total_slots as f64
    }

    /// 提取结构化想法 - 从问答对中构建结构化数据
    async fn extract_structured_idea(&self, qa_pairs: &[QAPair]) -> Option<StructuredIdea> {
        let mut structured = StructuredIdea {
            target: None,
            stakeholders: Vec::new(),
            constraints: std::collections::HashMap::new(),
            deliverables: Vec::new(),
            success_metrics: Vec::new(),
            risks_assumptions: Vec::new(),
        };

        for qa in qa_pairs {
            if let Some(answer) = &qa.answer {
                match qa.slot_type {
                    SlotType::Target => {
                        structured.target = Some(answer.clone());
                    }
                    SlotType::Stakeholder => {
                        // 解析多个利益相关者
                        let stakeholders: Vec<String> = answer
                            .split(&[',', '、', '；', ';'])
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        structured.stakeholders.extend(stakeholders);
                    }
                    SlotType::Constraints => {
                        // 根据问题类型分类约束
                        if qa.question.contains("预算") || qa.question.contains("时间") {
                            structured
                                .constraints
                                .insert("resource".to_string(), answer.clone());
                        } else {
                            structured
                                .constraints
                                .insert("technical".to_string(), answer.clone());
                        }
                    }
                    SlotType::Deliverable => {
                        let deliverables: Vec<String> = answer
                            .split(&[',', '、', '；', ';'])
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        structured.deliverables.extend(deliverables);
                    }
                    SlotType::Metrics => {
                        let metrics: Vec<String> = answer
                            .split(&[',', '、', '；', ';'])
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        structured.success_metrics.extend(metrics);
                    }
                    SlotType::RiskAssumptions => {
                        let risks: Vec<String> = answer
                            .split(&[',', '、', '；', ';'])
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        structured.risks_assumptions.extend(risks);
                    }
                }
            }
        }

        Some(structured)
    }

    /// 判断是否应该停止澄清过程
    fn should_stop_clarification(&self, clarification: &Clarification) -> bool {
        // 条件1: 所有槽位都已填充
        if clarification.open_slots.is_empty() {
            return true;
        }

        // 条件2: 置信度足够高(超过70%)
        if clarification.confidence >= 0.7 {
            return true;
        }

        // 条件3: 至少回答了核心问题（目标、利益相关者、交付物）
        let core_slots_answered = clarification
            .qa_pairs
            .iter()
            .filter(|qa| qa.answer.is_some())
            .filter(|qa| {
                matches!(
                    qa.slot_type,
                    SlotType::Target | SlotType::Stakeholder | SlotType::Deliverable
                )
            })
            .count();

        core_slots_answered >= 3
    }
}

#[async_trait]
impl Agent for ClarifierAgent {
    fn name(&self) -> &str {
        "Clarifier"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![AgentCapability::Clarification]
    }

    async fn execute(&self, context: AgentContext) -> Result<AgentResult> {
        tracing::info!("Clarifier executing for session: {}", context.session_id);

        // 获取或创建初始澄清状态
        let mut clarification = context.clarification.unwrap_or_else(|| {
            // 基于原始想法分析缺失的槽位
            let idea_seed = IdeaSeed {
                raw_text: "Sample idea".to_string(),
                context_hints: Vec::new(),
                domain: None,
            };

            Clarification {
                qa_pairs: Vec::new(),
                open_slots: vec![
                    SlotType::Target,
                    SlotType::Stakeholder,
                    SlotType::Constraints,
                    SlotType::Deliverable,
                    SlotType::Metrics,
                    SlotType::RiskAssumptions,
                ],
                confidence: 0.0,
                structured_idea: None,
            }
        });

        // 检查是否应该停止澄清
        if self.should_stop_clarification(&clarification) {
            tracing::info!(
                "Clarification process completed with confidence: {}",
                clarification.confidence
            );
            return Ok(AgentResult::Clarification(clarification));
        }

        // 生成新的问题
        if !clarification.open_slots.is_empty() {
            let idea_seed = IdeaSeed {
                raw_text: "Sample idea".to_string(), // 在实际实现中这应该从context获取
                context_hints: Vec::new(),
                domain: None,
            };

            let questions = self.generate_questions(&idea_seed, &clarification).await?;
            clarification.qa_pairs.extend(questions);
        }

        // 计算置信度
        clarification.confidence = self.analyze_slot_completeness(&clarification).await;

        // 如果置信度足够高，提取结构化想法
        if clarification.confidence >= 0.5 {
            clarification.structured_idea =
                self.extract_structured_idea(&clarification.qa_pairs).await;

            // 移除已完成的槽位
            let answered_slots: Vec<SlotType> = clarification
                .qa_pairs
                .iter()
                .filter(|qa| qa.answer.is_some())
                .map(|qa| qa.slot_type.clone())
                .collect();

            clarification
                .open_slots
                .retain(|slot| !answered_slots.contains(slot));
        }

        tracing::info!(
            "Clarification updated - confidence: {}, open_slots: {}",
            clarification.confidence,
            clarification.open_slots.len()
        );

        Ok(AgentResult::Clarification(clarification))
    }
}
