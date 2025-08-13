use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use anyhow::Result;

use crate::agents::{Agent, AgentCapability, AgentContext, AgentResult, Criticism};
use crate::core::data_structures::{IterationVersion, VerificationReport, Clarification, FactCheckStatus, RiskSeverity};
use crate::config::AppConfig;
use crate::models::{ModelManager, ChatRequest, ChatMessage};

pub struct SummarizerAgent {
    config: Arc<RwLock<AppConfig>>,
    model_manager: Arc<ModelManager>,
}

impl SummarizerAgent {
    pub async fn new(config: Arc<RwLock<AppConfig>>, model_manager: Arc<ModelManager>) -> Result<Self> {
        Ok(Self { 
            config,
            model_manager,
        })
    }

    /// 生成完整的优化会话总结报告（AI驱动）
    async fn generate_comprehensive_summary(&self, context: &AgentContext) -> Result<String> {
        // 分析所有结果
        let mut clarifications = Vec::new();
        let mut innovations = Vec::new();
        let mut criticisms = Vec::new();
        let mut iterations = Vec::new();
        let mut verifications = Vec::new();

        for result in &context.previous_results {
            match result {
                AgentResult::Clarification(clarification) => clarifications.push(clarification),
                AgentResult::Innovation(innovation_list) => innovations.extend(innovation_list.clone()),
                AgentResult::Criticism(criticism_list) => criticisms.extend(criticism_list.clone()),
                AgentResult::Synthesis(iteration) => iterations.push(iteration),
                AgentResult::Verification(verification) => verifications.push(verification),
                _ => {}
            }
        }

        // 构建AI总结请求
        let clarification_summary = if !clarifications.is_empty() {
            format!("澄清阶段完成，识别了 {} 个关键问题", clarifications.len())
        } else {
            "未进行澄清阶段".to_string()
        };

        let innovation_summary = format!("生成了 {} 个创新建议", innovations.len());
        let criticism_summary = format!("收到了 {} 个批评意见", criticisms.len());
        let iteration_summary = format!("完成了 {} 个迭代版本", iterations.len());
        let verification_summary = format!("进行了 {} 次验证", verifications.len());

        let innovations_text = innovations.iter()
            .take(10) // 只展示前10个
            .enumerate()
            .map(|(i, innovation)| format!("{}. {}", i + 1, innovation))
            .collect::<Vec<_>>()
            .join("\n");

        let criticisms_text = criticisms.iter()
            .take(5) // 只展示前5个
            .enumerate()
            .map(|(i, criticism)| format!("{}. [严重度:{:.1}] {}", i + 1, criticism.severity, criticism.message))
            .collect::<Vec<_>>()
            .join("\n");

        let iterations_text = iterations.iter()
            .enumerate()
            .map(|(i, iteration)| format!("版本 {}: {} (新颖性:{:.2}, 可行性:{:.2}, 一致性:{:.2})", 
                i + 1, iteration.summary, 
                iteration.scores.novelty, iteration.scores.feasibility, iteration.scores.coherence))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"你是一个专业的报告总结专家。请基于以下多智能体优化会话信息生成一份完整的总结报告：

会话ID：{}

流程摘要：
- {}
- {}
- {}
- {}
- {}

主要创新建议：
{}

关键批评意见：
{}

迭代版本：
{}

请生成一份结构化的Markdown格式报告，包含：

1. **执行摘要** - 整个优化过程的核心成果
2. **澄清阶段分析** - 问题识别和结构化过程
3. **创新建议汇总** - 按类别整理的建议
4. **批评意见分析** - 风险和改进点
5. **迭代版本评估** - 各版本的优缺点
6. **验证结果** - 质量检查结果
7. **最终建议** - 下一步行动计划
8. **风险提示** - 实施过程中需要注意的问题

要求：
- 使用专业的分析语言
- 提供具体的数据支撑
- 给出可操作的建议
- 突出关键洞察和价值点"#,
            context.session_id,
            clarification_summary,
            innovation_summary,
            criticism_summary,
            iteration_summary,
            verification_summary,
            innovations_text,
            criticisms_text,
            iterations_text
        );

        let request = ChatRequest {
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            model: "gpt-4".to_string(),
            temperature: Some(0.3),
            max_tokens: Some(3000),
        };

        match self.model_manager.chat(request).await {
            Ok(response) => {
                // 直接使用AI生成的报告
                Ok(response.content)
            }
            Err(e) => {
                tracing::warn!("AI总结失败，使用基础总结: {}", e);
                // 使用基础总结方法
                Ok(self.generate_basic_summary(context, &clarifications, &innovations, &criticisms, &iterations, &verifications).await?)
            }
        }
    }

    /// 生成基础总结报告（AI失败时的后备方案）
    async fn generate_basic_summary(
        &self,
        context: &AgentContext,
        clarifications: &[&Clarification],
        innovations: &[String],
        criticisms: &[Criticism],
        iterations: &[&IterationVersion],
        verifications: &[&VerificationReport],
    ) -> Result<String> {
        let mut summary = String::new();
        
        // 1. 标题和基本信息
        summary.push_str(&self.generate_header(context).await?);
        
        // 2. 执行摘要
        summary.push_str(&self.generate_executive_summary(clarifications, innovations, criticisms, iterations, verifications).await?);
        
        // 3. 分阶段分析
        if !clarifications.is_empty() {
            summary.push_str(&self.generate_clarification_summary(clarifications).await?);
        }
        
        if !innovations.is_empty() {
            summary.push_str(&self.generate_innovation_summary(innovations).await?);
        }
        
        if !criticisms.is_empty() {
            summary.push_str(&self.generate_criticism_summary(criticisms).await?);
        }
        
        if !iterations.is_empty() {
            summary.push_str(&self.generate_iteration_summary(iterations).await?);
        }
        
        if !verifications.is_empty() {
            summary.push_str(&self.generate_verification_summary(verifications).await?);
        }
        
        // 4. 最终建议和下一步
        summary.push_str(&self.generate_recommendations(iterations, verifications).await?);
        
        Ok(summary)
    }

    /// 生成报告头部
    async fn generate_header(&self, context: &AgentContext) -> Result<String> {
        let timestamp = chrono::Utc::now().format("%Y年%m月%d日 %H:%M UTC").to_string();
        
        Ok(format!(
            "# 智能概念优化报告\n\n**会话ID**: {}\n**生成时间**: {}\n**优化引擎**: magent 多智能体系统\n\n---\n\n",
            context.session_id,
            timestamp
        ))
    }

    /// 生成执行摘要
    async fn generate_executive_summary(
        &self, 
        clarifications: &[&Clarification],
        innovations: &[String],
        criticisms: &[Criticism],
        iterations: &[&IterationVersion],
        verifications: &[&VerificationReport]
    ) -> Result<String> {
        let mut summary = String::from("## 🎯 执行摘要\n\n");
        
        // 统计信息
        summary.push_str(&format!("本次优化会话通过多智能体协作，完成了从概念澄清到最终验证的完整流程：\n\n"));
        summary.push_str(&format!("- **澄清阶段**: 处理了 {} 个澄清项\n", clarifications.len()));
        summary.push_str(&format!("- **创新阶段**: 生成了 {} 个改进建议\n", innovations.len()));
        summary.push_str(&format!("- **批评阶段**: 提出了 {} 个批评意见\n", criticisms.len()));
        summary.push_str(&format!("- **综合阶段**: 产生了 {} 个迭代版本\n", iterations.len()));
        summary.push_str(&format!("- **验证阶段**: 完成了 {} 轮验证检查\n", verifications.len()));
        summary.push('\n');

        // 最终结果状态
        if let (Some(latest_iteration), Some(latest_verification)) = (iterations.last(), verifications.last()) {
            let verification_status = if latest_verification.passed { "✅ 通过" } else { "⚠️ 需要改进" };
            summary.push_str(&format!("**最终状态**: {}\n", verification_status));
            summary.push_str(&format!("**综合评分**: 新颖性 {:.1}/10, 可行性 {:.1}/10, 连贯性 {:.1}/10\n", 
                latest_iteration.scores.novelty * 10.0,
                latest_iteration.scores.feasibility * 10.0,
                latest_iteration.scores.coherence * 10.0
            ));
            summary.push_str(&format!("**验证置信度**: {:.1}%\n\n", latest_verification.confidence * 100.0));
        }

        summary.push_str("---\n\n");
        Ok(summary)
    }

    /// 生成澄清阶段总结
    async fn generate_clarification_summary(&self, clarifications: &[&Clarification]) -> Result<String> {
        let mut summary = String::from("## 💭 概念澄清阶段\n\n");
        
        for (i, clarification) in clarifications.iter().enumerate() {
            summary.push_str(&format!("### 澄清轮次 {}\n\n", i + 1));
            
            if !clarification.qa_pairs.is_empty() {
                summary.push_str("**问答对话**:\n");
                for (j, qa_pair) in clarification.qa_pairs.iter().enumerate() {
                    summary.push_str(&format!("{}. **Q**: {}\n", j + 1, qa_pair.question));
                    let answer_text = qa_pair.answer.as_ref().map(|s| s.as_str()).unwrap_or("待回答");
                    summary.push_str(&format!("   **A**: {}\n", answer_text));
                }
                summary.push('\n');
            }
            
            if !clarification.open_slots.is_empty() {
                summary.push_str("**待澄清项目**:\n");
                for (j, slot) in clarification.open_slots.iter().enumerate() {
                    let slot_name = match slot {
                        crate::core::data_structures::SlotType::Target => "目标",
                        crate::core::data_structures::SlotType::Stakeholder => "利益相关者",
                        crate::core::data_structures::SlotType::Constraints => "约束条件",
                        crate::core::data_structures::SlotType::Deliverable => "交付物",
                        crate::core::data_structures::SlotType::Metrics => "成功指标",
                        crate::core::data_structures::SlotType::RiskAssumptions => "风险假设",
                    };
                    summary.push_str(&format!("{}. {}\n", j + 1, slot_name));
                }
                summary.push('\n');
            }
            
            summary.push_str(&format!("**置信度**: {:.1}%\n\n", clarification.confidence * 100.0));
        }
        
        summary.push_str("---\n\n");
        Ok(summary)
    }

    /// 生成创新分析总结
    async fn generate_innovation_summary(&self, innovations: &[String]) -> Result<String> {
        let mut summary = String::from("## 💡 创新改进建议\n\n");
        
        summary.push_str(&format!("在创新阶段，系统生成了 {} 个改进建议，涵盖多个维度：\n\n", innovations.len()));
        
        // 按类型分类（简单的关键词分析）
        let mut tech_suggestions = Vec::new();
        let mut business_suggestions = Vec::new();
        let mut user_suggestions = Vec::new();
        let mut process_suggestions = Vec::new();
        let mut other_suggestions = Vec::new();

        for suggestion in innovations {
            let lower = suggestion.to_lowercase();
            if lower.contains("技术") || lower.contains("系统") || lower.contains("算法") {
                tech_suggestions.push(suggestion);
            } else if lower.contains("业务") || lower.contains("商业") || lower.contains("盈利") {
                business_suggestions.push(suggestion);
            } else if lower.contains("用户") || lower.contains("客户") || lower.contains("体验") {
                user_suggestions.push(suggestion);
            } else if lower.contains("流程") || lower.contains("过程") || lower.contains("管理") {
                process_suggestions.push(suggestion);
            } else {
                other_suggestions.push(suggestion);
            }
        }

        if !tech_suggestions.is_empty() {
            summary.push_str("### 🔧 技术改进\n");
            for suggestion in &tech_suggestions {
                summary.push_str(&format!("- {}\n", suggestion));
            }
            summary.push('\n');
        }

        if !business_suggestions.is_empty() {
            summary.push_str("### 💼 业务优化\n");
            for suggestion in &business_suggestions {
                summary.push_str(&format!("- {}\n", suggestion));
            }
            summary.push('\n');
        }

        if !user_suggestions.is_empty() {
            summary.push_str("### 👥 用户体验\n");
            for suggestion in &user_suggestions {
                summary.push_str(&format!("- {}\n", suggestion));
            }
            summary.push('\n');
        }

        if !process_suggestions.is_empty() {
            summary.push_str("### 🔄 流程改进\n");
            for suggestion in &process_suggestions {
                summary.push_str(&format!("- {}\n", suggestion));
            }
            summary.push('\n');
        }

        if !other_suggestions.is_empty() {
            summary.push_str("### 🎯 其他建议\n");
            for suggestion in &other_suggestions {
                summary.push_str(&format!("- {}\n", suggestion));
            }
            summary.push('\n');
        }

        summary.push_str("---\n\n");
        Ok(summary)
    }

    /// 生成批评分析总结
    async fn generate_criticism_summary(&self, criticisms: &[Criticism]) -> Result<String> {
        let mut summary = String::from("## 🔍 批评分析阶段\n\n");
        
        summary.push_str(&format!("批评阶段识别了 {} 个潜在问题和风险点：\n\n", criticisms.len()));
        
        // 按严重程度分类
        let critical_issues: Vec<_> = criticisms.iter().filter(|c| c.severity > 0.7).collect();
        let major_issues: Vec<_> = criticisms.iter().filter(|c| c.severity > 0.4 && c.severity <= 0.7).collect();
        let minor_issues: Vec<_> = criticisms.iter().filter(|c| c.severity <= 0.4).collect();

        if !critical_issues.is_empty() {
            summary.push_str("### 🚨 关键问题\n");
            for criticism in &critical_issues {
                summary.push_str(&format!("- **{}** (严重程度: {:.1}/10)\n", criticism.message, criticism.severity * 10.0));
            }
            summary.push('\n');
        }

        if !major_issues.is_empty() {
            summary.push_str("### ⚠️ 主要问题\n");
            for criticism in &major_issues {
                summary.push_str(&format!("- **{}** (严重程度: {:.1}/10)\n", criticism.message, criticism.severity * 10.0));
            }
            summary.push('\n');
        }

        if !minor_issues.is_empty() {
            summary.push_str("### 💡 改进建议\n");
            for criticism in &minor_issues {
                summary.push_str(&format!("- {}\n", criticism.message));
            }
            summary.push('\n');
        }

        summary.push_str("---\n\n");
        Ok(summary)
    }

    /// 生成迭代综合总结
    async fn generate_iteration_summary(&self, iterations: &[&IterationVersion]) -> Result<String> {
        let mut summary = String::from("## 🔄 综合迭代阶段\n\n");
        
        if iterations.len() == 1 {
            let iteration = iterations[0];
            summary.push_str("### 最终综合版本\n\n");
            summary.push_str(&format!("**版本摘要**: {}\n\n", iteration.summary));
            
            if !iteration.deltas.is_empty() {
                summary.push_str("**采纳的改进建议**:\n");
                for (i, delta) in iteration.deltas.iter().enumerate() {
                    summary.push_str(&format!("{}. {}\n", i + 1, delta));
                }
                summary.push('\n');
            }
            
            summary.push_str("**评分详情**:\n");
            summary.push_str(&format!("- 新颖性: {:.1}/10\n", iteration.scores.novelty * 10.0));
            summary.push_str(&format!("- 可行性: {:.1}/10\n", iteration.scores.feasibility * 10.0));
            summary.push_str(&format!("- 连贯性: {:.1}/10\n\n", iteration.scores.coherence * 10.0));
            
        } else if iterations.len() > 1 {
            summary.push_str(&format!("经过 {} 轮迭代，系统逐步优化了概念：\n\n", iterations.len()));
            
            for (i, iteration) in iterations.iter().enumerate() {
                summary.push_str(&format!("### 迭代 {} - {}\n", i + 1, iteration.summary));
                summary.push_str(&format!("- 改进数量: {}\n", iteration.deltas.len()));
                summary.push_str(&format!("- 综合评分: {:.1}/10\n\n", 
                    (iteration.scores.novelty + iteration.scores.feasibility + iteration.scores.coherence) / 3.0 * 10.0));
            }
        }
        
        summary.push_str("---\n\n");
        Ok(summary)
    }

    /// 生成验证结果总结
    async fn generate_verification_summary(&self, verifications: &[&VerificationReport]) -> Result<String> {
        let mut summary = String::from("## ✅ 验证结果\n\n");
        
        let latest_verification = verifications.last().unwrap();
        
        summary.push_str(&format!("**总体状态**: {}\n", 
            if latest_verification.passed { "✅ 验证通过" } else { "⚠️ 需要改进" }));
        summary.push_str(&format!("**验证置信度**: {:.1}%\n\n", latest_verification.confidence * 100.0));
        
        // 逻辑检查结果
        if !latest_verification.logic_checks.is_empty() {
            summary.push_str("### 🧠 逻辑一致性检查\n");
            for check in &latest_verification.logic_checks {
                let status = if check.passed { "✅" } else { "❌" };
                summary.push_str(&format!("- {} **{}**: {}\n", status, check.check_type, check.message));
            }
            summary.push('\n');
        }
        
        // 事实检查结果
        if !latest_verification.fact_checks.is_empty() {
            summary.push_str("### 📋 事实准确性检查\n");
            for check in &latest_verification.fact_checks {
                let status = match check.status {
                    FactCheckStatus::Supported => "✅",
                    FactCheckStatus::Partial => "⚠️",
                    FactCheckStatus::Unsupported => "❌",
                    FactCheckStatus::NeedClarification => "❓",
                };
                summary.push_str(&format!("- {} **{}** (置信度: {:.1}%)\n", 
                    status, check.claim, check.confidence * 100.0));
            }
            summary.push('\n');
        }
        
        // 风险评估
        if !latest_verification.risks.is_empty() {
            summary.push_str("### ⚠️ 风险评估\n");
            for risk in &latest_verification.risks {
                let severity_icon = match risk.severity {
                    RiskSeverity::Low => "🟢",
                    RiskSeverity::Medium => "🟡",
                    RiskSeverity::High => "🟠",
                    RiskSeverity::Critical => "🔴",
                };
                summary.push_str(&format!("- {} {}\n", severity_icon, risk.description));
                if let Some(mitigation) = &risk.mitigation {
                    summary.push_str(&format!("  *缓解措施*: {}\n", mitigation));
                }
            }
            summary.push('\n');
        }
        
        summary.push_str("---\n\n");
        Ok(summary)
    }

    /// 生成最终建议和下一步
    async fn generate_recommendations(&self, iterations: &[&IterationVersion], verifications: &[&VerificationReport]) -> Result<String> {
        let mut summary = String::from("## 🎯 建议与下一步\n\n");
        
        if let (Some(latest_iteration), Some(latest_verification)) = (iterations.last(), verifications.last()) {
            // 基于验证结果给出建议
            if latest_verification.passed {
                summary.push_str("### ✅ 实施建议\n\n");
                summary.push_str("概念已通过验证，建议进入实施阶段：\n\n");
                
                if !latest_iteration.deltas.is_empty() {
                    summary.push_str("**优先实施项目**:\n");
                    // 取前几个最重要的改进建议
                    let top_deltas = latest_iteration.deltas.iter().take(3);
                    for (i, delta) in top_deltas.enumerate() {
                        summary.push_str(&format!("{}. {}\n", i + 1, delta));
                    }
                    summary.push('\n');
                }
                
                summary.push_str("**建议实施步骤**:\n");
                summary.push_str("1. 制定详细的项目计划\n");
                summary.push_str("2. 分配必要的资源\n");
                summary.push_str("3. 建立里程碑和检查点\n");
                summary.push_str("4. 持续监控和调整\n\n");
                
            } else {
                summary.push_str("### ⚠️ 改进建议\n\n");
                summary.push_str("概念需要进一步优化，建议：\n\n");
                
                // 基于验证失败的原因给出具体建议
                let failed_logic_checks = latest_verification.logic_checks.iter()
                    .filter(|c| !c.passed)
                    .count();
                let critical_risks = latest_verification.risks.iter()
                    .filter(|r| matches!(r.severity, RiskSeverity::Critical | RiskSeverity::High))
                    .count();
                
                if failed_logic_checks > 0 {
                    summary.push_str("- 重新审视概念的逻辑一致性\n");
                }
                if critical_risks > 0 {
                    summary.push_str("- 制定更详细的风险缓解策略\n");
                }
                
                summary.push_str("- 考虑启动新一轮的对抗性优化\n");
                summary.push_str("- 寻求领域专家的额外输入\n\n");
            }
        }
        
        summary.push_str("### 📞 联系信息\n\n");
        summary.push_str("如需进一步讨论或启动新的优化会话，请使用会话ID联系系统。\n\n");
        
        summary.push_str("---\n\n");
        summary.push_str("*本报告由 magent 多智能体优化系统自动生成*\n");
        
        Ok(summary)
    }
}

#[async_trait]
impl Agent for SummarizerAgent {
    fn name(&self) -> &str {
        "Summarizer"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![AgentCapability::Summarization]
    }

    async fn execute(&self, context: AgentContext) -> Result<AgentResult> {
        tracing::info!("Summarizer executing for session: {}", context.session_id);

        // 生成完整的会话总结报告
        let summary = self.generate_comprehensive_summary(&context).await?;

        Ok(AgentResult::Summary(summary))
    }
}
