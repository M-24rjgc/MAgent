use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use anyhow::Result;

use crate::agents::{Agent, AgentCapability, AgentContext, AgentResult};
use crate::core::data_structures::{VerificationReport, LogicCheck, FactCheck, Risk, FactCheckStatus, RiskSeverity, IterationVersion, Evidence};
use crate::config::AppConfig;
use crate::models::{ModelManager, ChatRequest, ChatMessage};
use crate::storage::DataStore;

pub struct VerifierAgent {
    config: Arc<RwLock<AppConfig>>,
    storage: Arc<DataStore>,
    model_manager: Arc<ModelManager>,
}

impl VerifierAgent {
    pub async fn new(config: Arc<RwLock<AppConfig>>, storage: Arc<DataStore>, model_manager: Arc<ModelManager>) -> Result<Self> {
        Ok(Self { 
            config,
            storage,
            model_manager,
        })
    }

    /// 验证迭代版本的质量和一致性（AI驱动）
    async fn verify_iteration(&self, iteration: &IterationVersion) -> Result<VerificationReport> {
        // 构建AI验证请求
        let deltas_text = iteration.deltas.iter()
            .enumerate()
            .map(|(i, delta)| format!("{}. {}", i + 1, delta))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"你是一个专业的方案验证专家。请对以下迭代版本进行全面验证：

版本摘要：{}
推理过程：{}
创新建议：
{}

评分指标：
- 新颖性：{:.2}
- 可行性：{:.2}  
- 一致性：{:.2}

请进行全面验证，并以JSON格式返回：

{{
    "logic_checks": [
        {{
            "check_type": "consistency|completeness|coherence",
            "description": "检查描述",
            "passed": true,
            "message": "检查结果说明"
        }}
    ],
    "fact_checks": [
        {{
            "claim": "需要验证的声明",
            "evidence_summary": "证据摘要",
            "status": "verified|unverified|contradicted",
            "confidence": 0.85
        }}
    ],
    "risks": [
        {{
            "type": "implementation|market|technical|resource",
            "description": "风险描述",
            "severity": "low|medium|high|critical",
            "mitigation": "缓解措施建议"
        }}
    ],
    "overall_passed": true,
    "confidence": 0.88
}}

验证要求：
1. 逻辑一致性：检查建议间的逻辑关系
2. 事实验证：评估声明的合理性和支撑证据
3. 风险识别：识别实施过程中的潜在风险
4. 整体评估：给出通过状态和置信度"#,
            iteration.summary,
            iteration.rationale,
            deltas_text,
            iteration.scores.novelty,
            iteration.scores.feasibility,
            iteration.scores.coherence
        );

        let request = ChatRequest {
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            model: "gpt-4".to_string(),
            temperature: Some(0.2),
            max_tokens: Some(2000),
        };

        match self.model_manager.chat(request).await {
            Ok(response) => {
                // 解析AI响应
                if let Ok(verification_result) = self.parse_verification_response(&response.content) {
                    Ok(verification_result)
                } else {
                    // AI解析失败，使用基础验证
                    Ok(self.generate_basic_verification(iteration).await?)
                }
            }
            Err(e) => {
                tracing::warn!("AI验证失败，使用基础验证: {}", e);
                Ok(self.generate_basic_verification(iteration).await?)
            }
        }
    }

    /// 解析AI验证响应
    fn parse_verification_response(&self, response: &str) -> Result<VerificationReport> {
        use serde_json::Value;

        let json: Value = serde_json::from_str(response)?;
        
        // 解析逻辑检查
        let mut logic_checks = Vec::new();
        if let Some(checks) = json["logic_checks"].as_array() {
            for check in checks {
                logic_checks.push(LogicCheck {
                    check_type: check["check_type"].as_str().unwrap_or("consistency").to_string(),
                    description: check["description"].as_str().unwrap_or("").to_string(),
                    passed: check["passed"].as_bool().unwrap_or(true),
                    message: check["message"].as_str().unwrap_or("").to_string(),
                });
            }
        }

        // 解析事实检查
        let mut fact_checks = Vec::new();
        if let Some(checks) = json["fact_checks"].as_array() {
            for check in checks {
                let status = match check["status"].as_str().unwrap_or("supported") {
                    "supported" => FactCheckStatus::Supported,
                    "partial" => FactCheckStatus::Partial,
                    "unsupported" => FactCheckStatus::Unsupported,
                    _ => FactCheckStatus::NeedClarification,
                };

                fact_checks.push(FactCheck {
                    claim: check["claim"].as_str().unwrap_or("").to_string(),
                    evidence: vec![Evidence {
                        source_id: "AI分析".to_string(),
                        snippet: check["evidence_summary"].as_str().unwrap_or("").to_string(),
                        relevance: check["confidence"].as_f64().unwrap_or(0.7),
                        url: None,
                    }],
                    status,
                    confidence: check["confidence"].as_f64().unwrap_or(0.7),
                });
            }
        }

        // 解析风险
        let mut risks = Vec::new();
        if let Some(risk_array) = json["risks"].as_array() {
            for risk in risk_array {
                let severity = match risk["severity"].as_str().unwrap_or("medium") {
                    "low" => RiskSeverity::Low,
                    "high" => RiskSeverity::High,
                    "critical" => RiskSeverity::Critical,
                    _ => RiskSeverity::Medium,
                };

                risks.push(Risk {
                    description: risk["description"].as_str().unwrap_or("").to_string(),
                    severity,
                    mitigation: Some(risk["mitigation"].as_str().unwrap_or("").to_string()),
                });
            }
        }

        let passed = json["overall_passed"].as_bool().unwrap_or(true);
        let confidence = json["confidence"].as_f64().unwrap_or(0.7);

        Ok(VerificationReport {
            logic_checks,
            fact_checks,
            risks,
            passed,
            confidence,
        })
    }

    /// 生成基础验证结果（AI失败时的后备方案）
    async fn generate_basic_verification(&self, iteration: &IterationVersion) -> Result<VerificationReport> {
        let mut logic_checks = Vec::new();
        let mut fact_checks = Vec::new();
        let mut risks = Vec::new();

        // 1. 逻辑一致性检查
        logic_checks.extend(self.perform_logic_checks(iteration).await?);
        
        // 2. 事实准确性检查
        fact_checks.extend(self.perform_fact_checks(iteration).await?);
        
        // 3. 风险评估
        risks.extend(self.assess_risks(iteration).await?);

        // 4. 计算总体通过状态和置信度
        let passed = self.calculate_overall_pass(&logic_checks, &fact_checks, &risks).await?;
        let confidence = self.calculate_verification_confidence(&logic_checks, &fact_checks, &risks).await?;

        Ok(VerificationReport {
            logic_checks,
            fact_checks,
            risks,
            passed,
            confidence,
        })
    }

    /// 执行逻辑一致性检查
    async fn perform_logic_checks(&self, iteration: &IterationVersion) -> Result<Vec<LogicCheck>> {
        let mut checks = Vec::new();

        // 检查摘要与详细信息的一致性
        let summary_check = self.check_summary_consistency(iteration).await?;
        checks.push(summary_check);

        // 检查改进建议的逻辑连贯性
        let delta_coherence_check = self.check_delta_coherence(iteration).await?;
        checks.push(delta_coherence_check);

        // 检查评分的合理性
        let scoring_check = self.check_scoring_consistency(iteration).await?;
        checks.push(scoring_check);

        Ok(checks)
    }

    /// 检查摘要与详细信息的一致性
    async fn check_summary_consistency(&self, iteration: &IterationVersion) -> Result<LogicCheck> {
        let summary_length = iteration.summary.len();
        let deltas_count = iteration.deltas.len();
        let rationale_length = iteration.rationale.len();

        let passed = summary_length > 10 && 
                    deltas_count > 0 && 
                    rationale_length > 100 &&
                    !iteration.summary.contains("TODO") &&
                    !iteration.summary.contains("待定");

        let message = if passed {
            "摘要与详细信息保持一致".to_string()
        } else {
            format!("摘要可能不完整或与详细信息不符 (摘要长度: {}, 改进数量: {}, 推理长度: {})", 
                   summary_length, deltas_count, rationale_length)
        };

        Ok(LogicCheck {
            check_type: "摘要一致性检查".to_string(),
            description: "检查摘要是否与详细的改进建议和推理保持一致".to_string(),
            passed,
            message,
        })
    }

    /// 检查改进建议的逻辑连贯性
    async fn check_delta_coherence(&self, iteration: &IterationVersion) -> Result<LogicCheck> {
        let deltas = &iteration.deltas;
        
        if deltas.is_empty() {
            return Ok(LogicCheck {
                check_type: "改进建议连贯性检查".to_string(),
                description: "检查改进建议之间的逻辑连贯性".to_string(),
                passed: false,
                message: "没有找到任何改进建议".to_string(),
            });
        }

        // 检查是否有重复或矛盾的建议
        let mut contradictions = 0;
        let conflict_keywords = [
            ("增加", "减少"), ("添加", "删除"), ("简化", "复杂化"),
            ("集中", "分散"), ("自动化", "手工"), ("统一", "分离")
        ];

        for i in 0..deltas.len() {
            for j in i+1..deltas.len() {
                let delta1 = &deltas[i].to_lowercase();
                let delta2 = &deltas[j].to_lowercase();
                
                for (word1, word2) in &conflict_keywords {
                    if (delta1.contains(word1) && delta2.contains(word2)) ||
                       (delta1.contains(word2) && delta2.contains(word1)) {
                        contradictions += 1;
                    }
                }
            }
        }

        let passed = contradictions == 0;
        let message = if passed {
            "改进建议之间逻辑连贯，无明显矛盾".to_string()
        } else {
            format!("发现 {} 个潜在的矛盾改进建议", contradictions)
        };

        Ok(LogicCheck {
            check_type: "改进建议连贯性检查".to_string(),
            description: "检查改进建议之间是否存在矛盾或重复".to_string(),
            passed,
            message,
        })
    }

    /// 检查评分的合理性
    async fn check_scoring_consistency(&self, iteration: &IterationVersion) -> Result<LogicCheck> {
        let scores = &iteration.scores;
        
        // 检查评分范围
        let valid_range = scores.novelty >= 0.0 && scores.novelty <= 1.0 &&
                         scores.feasibility >= 0.0 && scores.feasibility <= 1.0 &&
                         scores.coherence >= 0.0 && scores.coherence <= 1.0;

        // 检查评分合理性：通常可行性和连贯性应该相对较高
        let reasonable_scores = scores.feasibility >= 0.3 && scores.coherence >= 0.4;

        let passed = valid_range && reasonable_scores;
        let message = if passed {
            format!("评分合理 (新颖性: {:.2}, 可行性: {:.2}, 连贯性: {:.2})", 
                   scores.novelty, scores.feasibility, scores.coherence)
        } else {
            format!("评分可能不合理 (新颖性: {:.2}, 可行性: {:.2}, 连贯性: {:.2})", 
                   scores.novelty, scores.feasibility, scores.coherence)
        };

        Ok(LogicCheck {
            check_type: "评分一致性检查".to_string(),
            description: "检查各项评分是否在合理范围内".to_string(),
            passed,
            message,
        })
    }

    /// 执行事实准确性检查
    async fn perform_fact_checks(&self, iteration: &IterationVersion) -> Result<Vec<FactCheck>> {
        let mut checks = Vec::new();

        // 检查技术可行性声明
        let tech_feasibility = self.check_technical_claims(iteration).await?;
        checks.push(tech_feasibility);

        // 检查业务逻辑声明
        let business_logic = self.check_business_claims(iteration).await?;
        checks.push(business_logic);

        // 检查资源需求声明
        let resource_claims = self.check_resource_claims(iteration).await?;
        checks.push(resource_claims);

        Ok(checks)
    }

    /// 检查技术相关声明
    async fn check_technical_claims(&self, iteration: &IterationVersion) -> Result<FactCheck> {
        let rationale = &iteration.rationale.to_lowercase();
        let deltas_text = iteration.deltas.join(" ").to_lowercase();
        
        // 查找技术相关的声明
        let tech_keywords = ["技术", "算法", "系统", "架构", "开发", "实现", "集成"];
        let has_tech_claims = tech_keywords.iter().any(|&word| 
            rationale.contains(word) || deltas_text.contains(word)
        );

        if !has_tech_claims {
            return Ok(FactCheck {
                claim: "无技术相关声明".to_string(),
                evidence: vec![Evidence {
                    source_id: "analysis".to_string(),
                    snippet: "未发现技术相关声明".to_string(),
                    relevance: 1.0,
                    url: None,
                }],
                status: FactCheckStatus::NeedClarification,
                confidence: 1.0,
            });
        }

        // 简单的技术可行性评估
        let complexity_indicators = ["复杂", "困难", "挑战", "风险"];
        let complexity_mentions = complexity_indicators.iter()
            .filter(|&&word| rationale.contains(word) || deltas_text.contains(word))
            .count();

        let (status, confidence) = if complexity_mentions == 0 {
            (FactCheckStatus::Supported, 0.8)
        } else if complexity_mentions <= 2 {
            (FactCheckStatus::Partial, 0.6)
        } else {
            (FactCheckStatus::Unsupported, 0.4)
        };

        Ok(FactCheck {
            claim: "技术方案具有可行性".to_string(),
            evidence: vec![
                Evidence {
                    source_id: "analysis".to_string(),
                    snippet: format!("发现 {} 个技术关键词", tech_keywords.len()),
                    relevance: 0.8,
                    url: None,
                },
                Evidence {
                    source_id: "analysis".to_string(),
                    snippet: format!("发现 {} 个复杂性指标", complexity_mentions),
                    relevance: 0.7,
                    url: None,
                },
            ],
            status,
            confidence,
        })
    }

    /// 检查业务相关声明
    async fn check_business_claims(&self, iteration: &IterationVersion) -> Result<FactCheck> {
        let content = format!("{} {}", iteration.rationale, iteration.deltas.join(" ")).to_lowercase();
        
        let business_keywords = ["成本", "收益", "市场", "用户", "客户", "业务", "盈利"];
        let business_mentions = business_keywords.iter()
            .filter(|&&word| content.contains(word))
            .count();

        let (status, confidence) = if business_mentions >= 3 {
            (FactCheckStatus::Supported, 0.8)
        } else if business_mentions >= 1 {
            (FactCheckStatus::Partial, 0.6)
        } else {
            (FactCheckStatus::NeedClarification, 0.9)
        };

        Ok(FactCheck {
            claim: "业务逻辑合理".to_string(),
            evidence: vec![
                Evidence {
                    source_id: "analysis".to_string(),
                    snippet: format!("发现 {} 个业务相关关键词", business_mentions),
                    relevance: 0.8,
                    url: None,
                },
            ],
            status,
            confidence,
        })
    }

    /// 检查资源需求声明
    async fn check_resource_claims(&self, iteration: &IterationVersion) -> Result<FactCheck> {
        let content = format!("{} {}", iteration.rationale, iteration.deltas.join(" ")).to_lowercase();
        
        let resource_keywords = ["时间", "人力", "资源", "预算", "成本", "投入"];
        let resource_mentions = resource_keywords.iter()
            .filter(|&&word| content.contains(word))
            .count();

        let (status, confidence) = if resource_mentions >= 2 {
            (FactCheckStatus::Supported, 0.7)
        } else if resource_mentions >= 1 {
            (FactCheckStatus::Partial, 0.5)
        } else {
            (FactCheckStatus::NeedClarification, 0.8)
        };

        Ok(FactCheck {
            claim: "资源需求评估充分".to_string(),
            evidence: vec![
                Evidence {
                    source_id: "analysis".to_string(),
                    snippet: format!("发现 {} 个资源相关关键词", resource_mentions),
                    relevance: 0.7,
                    url: None,
                },
            ],
            status,
            confidence,
        })
    }

    /// 评估风险
    async fn assess_risks(&self, iteration: &IterationVersion) -> Result<Vec<Risk>> {
        let mut risks = Vec::new();

        // 评估实施复杂度风险
        risks.push(self.assess_implementation_complexity_risk(iteration).await?);

        // 评估资源风险
        risks.push(self.assess_resource_risk(iteration).await?);

        // 评估时间风险
        risks.push(self.assess_timeline_risk(iteration).await?);

        Ok(risks)
    }

    /// 评估实施复杂度风险
    async fn assess_implementation_complexity_risk(&self, iteration: &IterationVersion) -> Result<Risk> {
        let content = format!("{} {}", iteration.rationale, iteration.deltas.join(" ")).to_lowercase();
        
        let complexity_indicators = [
            "复杂", "困难", "挑战性", "多步骤", "依赖", "集成", "协调", "同步"
        ];
        
        let complexity_score = complexity_indicators.iter()
            .filter(|&&word| content.contains(word))
            .count();

        let (severity, mitigation) = match complexity_score {
            0..=1 => (RiskSeverity::Low, "复杂度较低，按标准流程实施即可"),
            2..=3 => (RiskSeverity::Medium, "建议分阶段实施，加强项目管理"),
            4..=5 => (RiskSeverity::High, "需要详细的实施计划和风险应对策略"),
            _ => (RiskSeverity::Critical, "复杂度极高，建议重新评估方案可行性"),
        };

        Ok(Risk {
            description: format!("实施复杂度风险 (复杂度指标: {})", complexity_score),
            severity,
            mitigation: Some(mitigation.to_string()),
        })
    }

    /// 评估资源风险
    async fn assess_resource_risk(&self, iteration: &IterationVersion) -> Result<Risk> {
        let deltas_count = iteration.deltas.len();
        let content = format!("{} {}", iteration.rationale, iteration.deltas.join(" ")).to_lowercase();
        
        let resource_demanding_keywords = ["大量", "众多", "全面", "所有", "整体", "系统性"];
        let resource_demand_score = resource_demanding_keywords.iter()
            .filter(|&&word| content.contains(word))
            .count();

        let total_risk_score = deltas_count + resource_demand_score * 2;

        let (severity, mitigation) = match total_risk_score {
            0..=3 => (RiskSeverity::Low, "资源需求在可控范围内"),
            4..=6 => (RiskSeverity::Medium, "需要合理规划资源分配"),
            7..=10 => (RiskSeverity::High, "资源需求较大，建议分期投入"),
            _ => (RiskSeverity::Critical, "资源需求超出一般项目范围，需重新评估"),
        };

        Ok(Risk {
            description: format!("资源需求风险 (改进数量: {}, 需求强度: {})", deltas_count, resource_demand_score),
            severity,
            mitigation: Some(mitigation.to_string()),
        })
    }

    /// 评估时间风险
    async fn assess_timeline_risk(&self, iteration: &IterationVersion) -> Result<Risk> {
        let content = format!("{} {}", iteration.rationale, iteration.deltas.join(" ")).to_lowercase();
        
        let urgency_keywords = ["紧急", "立即", "马上", "尽快", "短期"];
        let long_term_keywords = ["长期", "逐步", "分阶段", "持续"];
        
        let urgency_score = urgency_keywords.iter()
            .filter(|&&word| content.contains(word))
            .count();
        let long_term_score = long_term_keywords.iter()
            .filter(|&&word| content.contains(word))
            .count();

        let (severity, mitigation) = if urgency_score > long_term_score && urgency_score > 2 {
            (RiskSeverity::High, "时间要求紧迫，需要加快实施速度或调整范围")
        } else if urgency_score > 0 && long_term_score == 0 {
            (RiskSeverity::Medium, "建议制定详细的时间计划")
        } else {
            (RiskSeverity::Low, "时间安排合理")
        };

        Ok(Risk {
            description: format!("时间安排风险 (紧急度: {}, 长期规划: {})", urgency_score, long_term_score),
            severity,
            mitigation: Some(mitigation.to_string()),
        })
    }

    /// 计算总体通过状态
    async fn calculate_overall_pass(&self, logic_checks: &[LogicCheck], fact_checks: &[FactCheck], risks: &[Risk]) -> Result<bool> {
        // 检查逻辑检查通过率
        let logic_pass_rate = logic_checks.iter().filter(|c| c.passed).count() as f64 / logic_checks.len() as f64;
        
        // 检查事实检查通过率
        let fact_pass_count = fact_checks.iter()
            .filter(|c| matches!(c.status, FactCheckStatus::Supported | FactCheckStatus::NeedClarification))
            .count();
        let fact_pass_rate = if fact_checks.is_empty() { 1.0 } else { fact_pass_count as f64 / fact_checks.len() as f64 };
        
        // 检查关键风险
        let has_critical_risks = risks.iter().any(|r| matches!(r.severity, RiskSeverity::Critical));
        
        // 总体通过条件：逻辑检查通过率 >= 80%，事实检查通过率 >= 60%，无关键风险
        Ok(logic_pass_rate >= 0.8 && fact_pass_rate >= 0.6 && !has_critical_risks)
    }

    /// 计算验证置信度
    async fn calculate_verification_confidence(&self, logic_checks: &[LogicCheck], fact_checks: &[FactCheck], risks: &[Risk]) -> Result<f64> {
        // 逻辑检查置信度
        let logic_confidence = if logic_checks.is_empty() { 0.5 } else {
            logic_checks.iter().filter(|c| c.passed).count() as f64 / logic_checks.len() as f64
        };
        
        // 事实检查置信度
        let fact_confidence = if fact_checks.is_empty() { 0.5 } else {
            fact_checks.iter().map(|c| c.confidence).sum::<f64>() / fact_checks.len() as f64
        };
        
        // 风险置信度（风险越小置信度越高）
        let risk_confidence = if risks.is_empty() { 0.8 } else {
            let avg_risk_severity = risks.iter().map(|r| match r.severity {
                RiskSeverity::Low => 0.9,
                RiskSeverity::Medium => 0.7,
                RiskSeverity::High => 0.4,
                RiskSeverity::Critical => 0.1,
            }).sum::<f64>() / risks.len() as f64;
            avg_risk_severity
        };
        
        // 加权平均
        let overall_confidence = (logic_confidence * 0.4 + fact_confidence * 0.3 + risk_confidence * 0.3)
            .max(0.0).min(1.0);
        
        Ok(overall_confidence)
    }
}

#[async_trait]
impl Agent for VerifierAgent {
    fn name(&self) -> &str {
        "Verifier"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![AgentCapability::Verification]
    }

    async fn execute(&self, context: AgentContext) -> Result<AgentResult> {
        tracing::info!("Verifier executing for session: {}", context.session_id);

        // 从previous_results中提取综合后的迭代版本
        let mut iteration_to_verify = None;
        
        for result in &context.previous_results {
            if let AgentResult::Synthesis(iteration) = result {
                iteration_to_verify = Some(iteration);
                break;
            }
        }

        let iteration = match iteration_to_verify {
            Some(iter) => iter,
            None => {
                // 如果没有找到综合结果，创建一个基本的验证报告
                let report = VerificationReport {
                    logic_checks: vec![
                        LogicCheck {
                            check_type: "输入验证".to_string(),
                            description: "检查是否有可验证的迭代版本".to_string(),
                            passed: false,
                            message: "未找到需要验证的综合迭代版本".to_string(),
                        },
                    ],
                    fact_checks: Vec::new(),
                    risks: Vec::new(),
                    passed: false,
                    confidence: 0.0,
                };
                return Ok(AgentResult::Verification(report));
            }
        };

        // 执行完整的验证流程
        let report = self.verify_iteration(iteration).await?;
        
        Ok(AgentResult::Verification(report))
    }
}
