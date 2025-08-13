use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use anyhow::Result;

use crate::agents::{Agent, AgentCapability, AgentContext, AgentResult, Criticism, CriticismCategory};
use crate::core::data_structures::*;
use crate::config::AppConfig;

pub struct CriticAgent {
    config: Arc<RwLock<AppConfig>>,
}

/// 批判分析维度
#[derive(Debug, Clone, PartialEq)]
pub enum CriticalDimension {
    Logic,           // 逻辑一致性
    Feasibility,     // 可行性
    Resource,        // 资源需求
    Risk,           // 风险评估
    Timeline,       // 时间线
    Stakeholder,    // 利益相关者影响
    Ethics,         // 道德伦理
    Market,         // 市场环境
    Technical,      // 技术实现
    Legal,          // 法律合规
}

/// 详细的批判报告
#[derive(Debug, Clone)]
pub struct DetailedCriticism {
    pub criticism: Criticism,
    pub dimension: CriticalDimension,
    pub evidence: Vec<String>,       // 支撑证据
    pub counter_arguments: Vec<String>, // 反驳论据
    pub suggestions: Vec<String>,    // 改进建议
    pub impact_analysis: String,     // 影响分析
}

impl CriticAgent {
    pub async fn new(config: Arc<RwLock<AppConfig>>) -> Result<Self> {
        Ok(Self { config })
    }

    /// 对创新Delta进行全面的批判分析
    async fn analyze_deltas(&self, deltas: &[String], structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        for (index, delta) in deltas.iter().enumerate() {
            // 对每个Delta进行多维度批判分析
            criticisms.extend(self.analyze_single_delta(index, delta, structured_idea).await?);
        }
        
        // 全局一致性检查
        criticisms.extend(self.analyze_global_consistency(deltas, structured_idea).await?);
        
        // 按严重程度排序
        criticisms.sort_by(|a, b| b.criticism.severity.partial_cmp(&a.criticism.severity).unwrap());
        
        Ok(criticisms)
    }
    
    /// 分析单个Delta
    async fn analyze_single_delta(&self, index: usize, delta: &str, structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        // 逻辑一致性检查
        criticisms.extend(self.check_logic_consistency(index, delta, structured_idea).await?);
        
        // 可行性分析
        criticisms.extend(self.check_feasibility(index, delta, structured_idea).await?);
        
        // 资源需求评估
        criticisms.extend(self.check_resource_requirements(index, delta, structured_idea).await?);
        
        // 风险识别
        criticisms.extend(self.check_risks(index, delta, structured_idea).await?);
        
        // 时间线合理性
        criticisms.extend(self.check_timeline(index, delta, structured_idea).await?);
        
        // 利益相关者影响
        criticisms.extend(self.check_stakeholder_impact(index, delta, structured_idea).await?);
        
        // 技术实现检查
        criticisms.extend(self.check_technical_implementation(index, delta).await?);
        
        // 市场环境分析
        criticisms.extend(self.check_market_conditions(index, delta).await?);
        
        Ok(criticisms)
    }
    
    /// 逻辑一致性检查
    async fn check_logic_consistency(&self, index: usize, delta: &str, structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        // 检查是否与目标一致
        if let Some(idea) = structured_idea {
            if let Some(target) = &idea.target {
                if self.is_delta_conflicting_with_target(delta, target) {
                    criticisms.push(DetailedCriticism {
                        criticism: Criticism {
                            delta_index: index,
                            category: CriticismCategory::LogicFlaw,
                            message: "该创新建议与既定目标存在逻辑冲突".to_string(),
                            severity: 0.8,
                        },
                        dimension: CriticalDimension::Logic,
                        evidence: vec![
                            format!("既定目标：{}", target),
                            format!("创新建议：{}", delta),
                        ],
                        counter_arguments: vec![
                            "可能存在隐含的协同效应".to_string(),
                            "目标可能需要重新定义范围".to_string(),
                        ],
                        suggestions: vec![
                            "重新评估目标与创新的关系".to_string(),
                            "调整创新方向以符合核心目标".to_string(),
                        ],
                        impact_analysis: "目标与创新的不一致可能导致资源分散和执行混乱".to_string(),
                    });
                }
            }
        }
        
        // 检查内在逻辑矛盾
        if self.has_internal_contradiction(delta) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::LogicFlaw,
                    message: "创新建议内部存在逻辑矛盾".to_string(),
                    severity: 0.7,
                },
                dimension: CriticalDimension::Logic,
                evidence: vec![format!("矛盾表述：{}", delta)],
                counter_arguments: vec!["可能是表述不够清晰，而非逻辑错误".to_string()],
                suggestions: vec![
                    "澄清表述，消除歧义".to_string(),
                    "分解复杂建议为多个简单步骤".to_string(),
                ],
                impact_analysis: "逻辑矛盾会导致执行时的决策困难和方向不明".to_string(),
            });
        }
        
        Ok(criticisms)
    }
    
    /// 可行性分析
    async fn check_feasibility(&self, index: usize, delta: &str, structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        // 技术可行性
        if self.is_technically_unfeasible(delta) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::FeasibilityIssue,
                    message: "技术实现存在重大可行性问题".to_string(),
                    severity: 0.9,
                },
                dimension: CriticalDimension::Feasibility,
                evidence: vec![
                    "涉及尚未成熟的技术".to_string(),
                    "实施复杂度超出常规项目范畴".to_string(),
                ],
                counter_arguments: vec![
                    "技术发展迅速，可能性在增加".to_string(),
                    "可以通过分阶段实施降低风险".to_string(),
                ],
                suggestions: vec![
                    "降低技术复杂度要求".to_string(),
                    "寻找替代技术方案".to_string(),
                    "建立技术预研阶段".to_string(),
                ],
                impact_analysis: "技术不可行将导致项目失败和资源浪费".to_string(),
            });
        }
        
        // 组织可行性
        if let Some(idea) = structured_idea {
            if self.exceeds_organizational_capacity(delta, &idea.stakeholders) {
                criticisms.push(DetailedCriticism {
                    criticism: Criticism {
                        delta_index: index,
                        category: CriticismCategory::FeasibilityIssue,
                        message: "超出组织执行能力范围".to_string(),
                        severity: 0.7,
                    },
                    dimension: CriticalDimension::Feasibility,
                    evidence: vec![
                        format!("当前团队规模：{} 人", idea.stakeholders.len()),
                        "建议需要大规模协调".to_string(),
                    ],
                    counter_arguments: vec![
                        "可以通过外部合作解决".to_string(),
                        "团队能力可以通过培训提升".to_string(),
                    ],
                    suggestions: vec![
                        "分解为更小的执行单元".to_string(),
                        "寻求外部合作伙伴".to_string(),
                        "制定能力建设计划".to_string(),
                    ],
                    impact_analysis: "能力不匹配将导致执行质量下降和时间延误".to_string(),
                });
            }
        }
        
        Ok(criticisms)
    }
    
    /// 资源需求评估
    async fn check_resource_requirements(&self, index: usize, delta: &str, structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        if self.requires_excessive_resources(delta) {
            let mut severity = 0.6;
            let mut evidence = vec!["预计需要大量资源投入".to_string()];
            
            // 如果有约束信息，进行更精确的评估
            if let Some(idea) = structured_idea {
                if let Some(resource_constraint) = idea.constraints.get("resource") {
                    evidence.push(format!("现有资源约束：{}", resource_constraint));
                    severity = 0.8; // 有明确约束时，资源问题更严重
                }
            }
            
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::ResourceConstraint,
                    message: "资源需求可能超出承受能力".to_string(),
                    severity,
                },
                dimension: CriticalDimension::Resource,
                evidence,
                counter_arguments: vec![
                    "高回报可能证明投资合理".to_string(),
                    "可以通过融资解决资源问题".to_string(),
                ],
                suggestions: vec![
                    "详细计算投资回报率".to_string(),
                    "寻找降低成本的替代方案".to_string(),
                    "考虑分阶段投资".to_string(),
                ],
                impact_analysis: "资源不足将导致项目质量下降或无法完成".to_string(),
            });
        }
        
        Ok(criticisms)
    }
    
    /// 风险识别
    async fn check_risks(&self, index: usize, delta: &str, _structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        // 技术风险
        if self.has_high_technical_risk(delta) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::RiskConcern,
                    message: "存在显著技术风险".to_string(),
                    severity: 0.7,
                },
                dimension: CriticalDimension::Risk,
                evidence: vec![
                    "依赖未验证的技术".to_string(),
                    "技术复杂度高".to_string(),
                ],
                counter_arguments: vec![
                    "可以通过原型验证降低风险".to_string(),
                    "技术风险是创新的必然代价".to_string(),
                ],
                suggestions: vec![
                    "建立技术风险评估机制".to_string(),
                    "准备备选技术方案".to_string(),
                    "设立技术验证里程碑".to_string(),
                ],
                impact_analysis: "技术风险可能导致项目延期或失败".to_string(),
            });
        }
        
        // 市场风险
        if self.has_market_risk(delta) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::RiskConcern,
                    message: "市场接受度存在不确定性".to_string(),
                    severity: 0.6,
                },
                dimension: CriticalDimension::Market,
                evidence: vec![
                    "市场需求未充分验证".to_string(),
                    "竞争环境复杂".to_string(),
                ],
                counter_arguments: vec![
                    "创新往往需要教育市场".to_string(),
                    "先发优势可能带来竞争壁垒".to_string(),
                ],
                suggestions: vec![
                    "进行市场调研和用户验证".to_string(),
                    "制定市场教育策略".to_string(),
                    "准备快速调整方案".to_string(),
                ],
                impact_analysis: "市场风险可能导致产品无法获得预期收益".to_string(),
            });
        }
        
        Ok(criticisms)
    }
    
    /// 时间线合理性检查
    async fn check_timeline(&self, index: usize, delta: &str, _structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        if self.has_unrealistic_timeline(delta) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::FeasibilityIssue,
                    message: "时间线可能过于乐观".to_string(),
                    severity: 0.5,
                },
                dimension: CriticalDimension::Timeline,
                evidence: vec![
                    "涉及复杂流程改造".to_string(),
                    "需要多方协调配合".to_string(),
                ],
                counter_arguments: vec![
                    "有经验团队可能加快进度".to_string(),
                    "并行执行可以压缩时间".to_string(),
                ],
                suggestions: vec![
                    "增加时间缓冲".to_string(),
                    "识别关键路径".to_string(),
                    "制定风险应对预案".to_string(),
                ],
                impact_analysis: "时间线不现实可能导致质量妥协和团队压力".to_string(),
            });
        }
        
        Ok(criticisms)
    }
    
    /// 利益相关者影响分析
    async fn check_stakeholder_impact(&self, index: usize, delta: &str, structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        if let Some(idea) = structured_idea {
            if self.may_negatively_impact_stakeholders(delta, &idea.stakeholders) {
                criticisms.push(DetailedCriticism {
                    criticism: Criticism {
                        delta_index: index,
                        category: CriticismCategory::StakeholderConcern,
                        message: "可能对关键利益相关者产生负面影响".to_string(),
                        severity: 0.6,
                    },
                    dimension: CriticalDimension::Stakeholder,
                    evidence: vec![
                        "可能改变现有工作流程".to_string(),
                        "可能影响既得利益".to_string(),
                    ],
                    counter_arguments: vec![
                        "长期来看对所有人都有益".to_string(),
                        "可以通过沟通化解担忧".to_string(),
                    ],
                    suggestions: vec![
                        "制定利益相关者沟通计划".to_string(),
                        "设计过渡期支持措施".to_string(),
                        "确保利益分配公平".to_string(),
                    ],
                    impact_analysis: "利益相关者阻力可能导致实施困难".to_string(),
                });
            }
        }
        
        Ok(criticisms)
    }
    
    /// 技术实现检查
    async fn check_technical_implementation(&self, index: usize, delta: &str) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        if self.lacks_technical_detail(delta) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::ImplementationGap,
                    message: "缺乏具体的技术实现路径".to_string(),
                    severity: 0.4,
                },
                dimension: CriticalDimension::Technical,
                evidence: vec!["技术实现细节不明确".to_string()],
                counter_arguments: vec!["概念阶段不需要过多技术细节".to_string()],
                suggestions: vec![
                    "补充技术架构设计".to_string(),
                    "识别关键技术挑战".to_string(),
                ],
                impact_analysis: "技术路径不清晰可能导致执行偏差".to_string(),
            });
        }
        
        Ok(criticisms)
    }
    
    /// 市场环境分析
    async fn check_market_conditions(&self, index: usize, delta: &str) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        if self.ignores_market_reality(delta) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: index,
                    category: CriticismCategory::MarketMismatch,
                    message: "可能未充分考虑市场现实".to_string(),
                    severity: 0.5,
                },
                dimension: CriticalDimension::Market,
                evidence: vec!["缺乏市场环境分析".to_string()],
                counter_arguments: vec!["创新有时需要创造新市场".to_string()],
                suggestions: vec![
                    "进行市场环境调研".to_string(),
                    "分析竞争对手策略".to_string(),
                ],
                impact_analysis: "市场环境不匹配可能导致商业失败".to_string(),
            });
        }
        
        Ok(criticisms)
    }
    
    /// 全局一致性分析
    async fn analyze_global_consistency(&self, deltas: &[String], structured_idea: Option<&StructuredIdea>) -> Result<Vec<DetailedCriticism>> {
        let mut criticisms = Vec::new();
        
        // 检查Delta之间的冲突
        if self.deltas_have_conflicts(deltas) {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: 999, // 全局问题用特殊索引
                    category: CriticismCategory::LogicFlaw,
                    message: "多个创新建议之间存在冲突".to_string(),
                    severity: 0.8,
                },
                dimension: CriticalDimension::Logic,
                evidence: vec!["发现相互矛盾的建议".to_string()],
                counter_arguments: vec!["不同阶段可以采用不同策略".to_string()],
                suggestions: vec![
                    "重新评估建议优先级".to_string(),
                    "制定分阶段实施计划".to_string(),
                ],
                impact_analysis: "冲突的建议会导致执行混乱和资源浪费".to_string(),
            });
        }
        
        // 检查整体复杂度
        if deltas.len() > 5 {
            criticisms.push(DetailedCriticism {
                criticism: Criticism {
                    delta_index: 998,
                    category: CriticismCategory::ComplexityIssue,
                    message: "创新建议过多，可能导致执行复杂度过高".to_string(),
                    severity: 0.6,
                },
                dimension: CriticalDimension::Feasibility,
                evidence: vec![format!("共有 {} 个创新建议", deltas.len())],
                counter_arguments: vec!["全面的创新有助于系统性改进".to_string()],
                suggestions: vec![
                    "按优先级分批实施".to_string(),
                    "确定核心创新重点".to_string(),
                ],
                impact_analysis: "过高复杂度可能导致执行困难和效果折扣".to_string(),
            });
        }
        
        Ok(criticisms)
    }
    
    // ================== 辅助判断方法 ==================
    
    fn is_delta_conflicting_with_target(&self, delta: &str, target: &str) -> bool {
        // 简单的关键词冲突检测
        let delta_lower = delta.to_lowercase();
        let target_lower = target.to_lowercase();
        
        // 检查是否有相反的关键词
        let conflicting_pairs = [
            ("增加", "减少"), ("扩大", "缩小"), ("快速", "缓慢"),
            ("简化", "复杂"), ("集中", "分散"), ("自动", "手动"),
        ];
        
        for (word1, word2) in conflicting_pairs {
            if (delta_lower.contains(word1) && target_lower.contains(word2)) ||
               (delta_lower.contains(word2) && target_lower.contains(word1)) {
                return true;
            }
        }
        
        false
    }
    
    fn has_internal_contradiction(&self, delta: &str) -> bool {
        let delta_lower = delta.to_lowercase();
        
        // 检查内部矛盾的关键词组合
        let contradictory_phrases = [
            ("提高效率", "增加人工"),
            ("降低成本", "提升质量"),
            ("快速实施", "深入调研"),
        ];
        
        for (phrase1, phrase2) in contradictory_phrases {
            if delta_lower.contains(phrase1) && delta_lower.contains(phrase2) {
                return true;
            }
        }
        
        false
    }
    
    fn is_technically_unfeasible(&self, delta: &str) -> bool {
        let high_risk_keywords = [
            "完全自动化", "100%准确", "零延迟", "无限扩展",
            "完美预测", "绝对安全", "永不失败"
        ];
        
        let delta_lower = delta.to_lowercase();
        high_risk_keywords.iter().any(|&keyword| delta_lower.contains(keyword))
    }
    
    fn exceeds_organizational_capacity(&self, delta: &str, stakeholders: &[String]) -> bool {
        let delta_lower = delta.to_lowercase();
        let requires_large_team = delta_lower.contains("大规模") || 
                                 delta_lower.contains("全面") ||
                                 delta_lower.contains("系统性");
        
        requires_large_team && stakeholders.len() < 3
    }
    
    fn requires_excessive_resources(&self, delta: &str) -> bool {
        let high_cost_keywords = [
            "大规模投资", "全面升级", "重构", "颠覆性",
            "平台化", "生态", "全球化"
        ];
        
        let delta_lower = delta.to_lowercase();
        high_cost_keywords.iter().any(|&keyword| delta_lower.contains(keyword))
    }
    
    fn has_high_technical_risk(&self, delta: &str) -> bool {
        let risky_keywords = [
            "ai", "机器学习", "区块链", "量子", "新技术",
            "未验证", "实验性", "前沿"
        ];
        
        let delta_lower = delta.to_lowercase();
        risky_keywords.iter().any(|&keyword| delta_lower.contains(keyword))
    }
    
    fn has_market_risk(&self, delta: &str) -> bool {
        let market_risk_keywords = [
            "颠覆", "革命性", "全新模式", "创造需求",
            "教育市场", "改变习惯"
        ];
        
        let delta_lower = delta.to_lowercase();
        market_risk_keywords.iter().any(|&keyword| delta_lower.contains(keyword))
    }
    
    fn has_unrealistic_timeline(&self, delta: &str) -> bool {
        let quick_keywords = ["快速", "立即", "即刻", "短期内"];
        let complex_keywords = ["全面", "系统性", "重构", "转型"];
        
        let delta_lower = delta.to_lowercase();
        let is_quick = quick_keywords.iter().any(|&kw| delta_lower.contains(kw));
        let is_complex = complex_keywords.iter().any(|&kw| delta_lower.contains(kw));
        
        is_quick && is_complex
    }
    
    fn may_negatively_impact_stakeholders(&self, delta: &str, stakeholders: &[String]) -> bool {
        let disruptive_keywords = [
            "替代", "自动化", "简化", "集中化", "标准化"
        ];
        
        let delta_lower = delta.to_lowercase();
        let is_disruptive = disruptive_keywords.iter().any(|&kw| delta_lower.contains(kw));
        
        is_disruptive && !stakeholders.is_empty()
    }
    
    fn lacks_technical_detail(&self, delta: &str) -> bool {
        let vague_keywords = [
            "提升", "优化", "改进", "增强", "升级"
        ];
        
        let technical_keywords = [
            "架构", "算法", "接口", "协议", "框架", "平台"
        ];
        
        let delta_lower = delta.to_lowercase();
        let is_vague = vague_keywords.iter().any(|&kw| delta_lower.contains(kw));
        let has_technical = technical_keywords.iter().any(|&kw| delta_lower.contains(kw));
        
        is_vague && !has_technical
    }
    
    fn ignores_market_reality(&self, delta: &str) -> bool {
        let idealistic_keywords = [
            "完美", "理想", "最优", "最佳", "无缺陷"
        ];
        
        let delta_lower = delta.to_lowercase();
        idealistic_keywords.iter().any(|&keyword| delta_lower.contains(keyword))
    }
    
    fn deltas_have_conflicts(&self, deltas: &[String]) -> bool {
        // 简单的冲突检测 - 检查是否有相反的动作
        let conflicting_actions = [
            ("集中", "分散"), ("扩大", "缩小"), ("增加", "减少"),
            ("自动化", "人工"), ("复杂", "简化")
        ];
        
        for i in 0..deltas.len() {
            for j in i+1..deltas.len() {
                let delta1 = deltas[i].to_lowercase();
                let delta2 = deltas[j].to_lowercase();
                
                for (action1, action2) in conflicting_actions {
                    if (delta1.contains(action1) && delta2.contains(action2)) ||
                       (delta1.contains(action2) && delta2.contains(action1)) {
                        return true;
                    }
                }
            }
        }
        
        false
    }
    
    /// 生成批判总结报告
    fn generate_criticism_summary(&self, criticisms: &[DetailedCriticism]) -> String {
        let mut summary = "🔍 批判分析报告\n\n".to_string();
        
        // 按严重程度分类
        let high_severity: Vec<_> = criticisms.iter().filter(|c| c.criticism.severity >= 0.7).collect();
        let medium_severity: Vec<_> = criticisms.iter().filter(|c| c.criticism.severity >= 0.4 && c.criticism.severity < 0.7).collect();
        let low_severity: Vec<_> = criticisms.iter().filter(|c| c.criticism.severity < 0.4).collect();
        
        summary.push_str(&format!("📊 问题分布：高风险 {} 个，中风险 {} 个，低风险 {} 个\n\n",
            high_severity.len(), medium_severity.len(), low_severity.len()));
        
        if !high_severity.is_empty() {
            summary.push_str("🚨 高风险问题：\n");
            for (i, criticism) in high_severity.iter().take(3).enumerate() {
                summary.push_str(&format!("{}. {} (严重程度: {:.1})\n", 
                    i + 1, criticism.criticism.message, criticism.criticism.severity));
                summary.push_str(&format!("   💡 建议：{}\n\n", 
                    criticism.suggestions.first().unwrap_or(&"需要进一步分析".to_string())));
            }
        }
        
        summary.push_str("📈 整体评估：");
        if high_severity.is_empty() {
            summary.push_str("风险可控，建议继续推进\n");
        } else if high_severity.len() <= 2 {
            summary.push_str("存在重要风险，需要重点关注\n");
        } else {
            summary.push_str("风险较高，建议重新评估方案\n");
        }
        
        summary
    }
}

#[async_trait]
impl Agent for CriticAgent {
    fn name(&self) -> &str {
        "Critic"
    }

    fn capabilities(&self) -> Vec<AgentCapability> {
        vec![AgentCapability::Criticism]
    }

    async fn execute(&self, context: AgentContext) -> Result<AgentResult> {
        tracing::info!("Critic executing for session: {}", context.session_id);

        // 获取Innovator的Delta输出
        let deltas = if let Some(innovation) = context.previous_results.iter()
            .find_map(|result| if let AgentResult::Innovation(deltas) = result { Some(deltas) } else { None }) {
            innovation.clone()
        } else {
            // 如果没有创新输入，创建示例数据用于测试
            vec![
                "[技术创新] 引入AI自动化技术提升效率".to_string(),
                "[商业模式] 采用订阅制商业模式".to_string(),
                "[用户体验] 设计个性化用户界面".to_string(),
            ]
        };

        // 获取结构化想法
        let structured_idea = context.clarification
            .as_ref()
            .and_then(|c| c.structured_idea.as_ref());

        // 进行详细的批判分析
        let detailed_criticisms = self.analyze_deltas(&deltas, structured_idea).await?;
        
        // 记录分析结果
        tracing::info!("Generated {} detailed criticisms", detailed_criticisms.len());
        
        // 生成总结报告
        let summary = self.generate_criticism_summary(&detailed_criticisms);
        tracing::info!("Criticism summary: {}", summary);
        
        // 转换为简单的Criticism格式用于输出
        let simple_criticisms: Vec<Criticism> = detailed_criticisms
            .into_iter()
            .map(|dc| dc.criticism)
            .collect();

        tracing::info!("Critic completed with {} criticisms", simple_criticisms.len());
        Ok(AgentResult::Criticism(simple_criticisms))
    }
}
