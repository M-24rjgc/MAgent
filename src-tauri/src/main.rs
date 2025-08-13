// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// 抑制开发期间的未使用代码警告
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::collections::HashMap;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::{mpsc, RwLock};

mod agents;
mod config;
mod core;
mod models;
mod storage;

#[cfg(test)]
mod tests;

use crate::agents::clarifier::ClarifierAgent;
use crate::agents::innovator::InnovatorAgent;
use crate::config::AppConfig;
use crate::core::data_structures::{IdeaSeed, StructuredIdea};
use crate::core::{AppState, SystemEvent};
use crate::models::ModelManager;
use crate::storage::DataStore;
use uuid::Uuid;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

/// 开始概念优化
#[tauri::command]
async fn start_concept_optimization(
    seed: String,
    _state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    println!("Starting concept optimization for: {}", seed);
    Ok(format!("优化已开始，想法: {}", seed))
}

/// 运行澄清智能体（实际AI调用）
#[tauri::command]
async fn run_clarification_ai(
    state: tauri::State<'_, AppState>,
    idea_content: String,
) -> Result<serde_json::Value, String> {
    println!("Running real AI clarification for idea: {}", idea_content);

    let idea_seed = IdeaSeed {
        raw_text: idea_content.clone(),
        context_hints: vec![],
        domain: None,
    };

    // 创建 ModelManager 实例
    let model_manager = ModelManager::new(state.config.clone());

    // 创建实际的 ClarifierAgent
    let clarifier = ClarifierAgent::new(state.config.clone(), Arc::new(model_manager))
        .await
        .map_err(|e| format!("创建澄清代理失败: {}", e))?;

    // 调用真正的澄清分析
    match clarifier.analyze_and_clarify(&idea_seed).await {
        Ok(clarification) => {
            let result = serde_json::json!({
                "status": "completed",
                "clarification": {
                    "questions": clarification.qa_pairs.iter().map(|q| {
                        serde_json::json!({
                            "question": q.question,
                            "type": format!("{:?}", q.slot_type),
                            "priority": "high"
                        })
                    }).collect::<Vec<_>>(),
                    "confidence": clarification.confidence,
                    "missing_slots": clarification.open_slots.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>(),
                    "structured_idea": {
                        "target": clarification.structured_idea.as_ref().and_then(|si| si.target.as_ref()).unwrap_or(&"未设定".to_string()),
                        "stakeholders": clarification.structured_idea.as_ref().map(|si| &si.stakeholders).unwrap_or(&vec![]),
                        "constraints": clarification.structured_idea.as_ref().map(|si| &si.constraints).unwrap_or(&std::collections::HashMap::new()),
                        "deliverables": clarification.structured_idea.as_ref().map(|si| &si.deliverables).unwrap_or(&vec![]),
                        "success_metrics": clarification.structured_idea.as_ref().map(|si| &si.success_metrics).unwrap_or(&vec![]),
                        "risk_assumptions": clarification.structured_idea.as_ref().map(|si| &si.risks_assumptions).unwrap_or(&vec![])
                    }
                }
            });
            Ok(result)
        }
        Err(e) => {
            eprintln!("Clarification error: {}", e);
            Err(format!("澄清分析失败: {}", e))
        }
    }
}

/// 运行创新智能体（实际AI调用）
#[tauri::command]
async fn run_innovation_ai(
    state: tauri::State<'_, AppState>,
    structured_idea_json: String,
) -> Result<serde_json::Value, String> {
    println!("Running real AI innovation analysis");

    // 解析结构化想法
    let structured_idea: StructuredIdea = serde_json::from_str(&structured_idea_json)
        .map_err(|e| format!("解析结构化想法失败: {}", e))?;

    // 创建 ModelManager 实例
    let model_manager = ModelManager::new(state.config.clone());

    // 创建实际的 InnovatorAgent
    let innovator = InnovatorAgent::new(state.config.clone(), Arc::new(model_manager))
        .await
        .map_err(|e| format!("创建创新代理失败: {}", e))?;

    // 调用真正的创新分析
    match innovator.generate_deltas(&structured_idea).await {
        Ok(deltas) => {
            let result = serde_json::json!({
                "status": "completed",
                "deltas": deltas.iter().map(|delta| {
                    serde_json::json!({
                        "dimension": format!("{:?}", delta.dimension),
                        "description": delta.content,
                        "reasoning": delta.reasoning,
                        "impact_score": delta.impact_level,
                        "feasibility": delta.feasibility,
                        "innovation_level": delta.innovation_score
                    })
                }).collect::<Vec<_>>(),
                "total_suggestions": deltas.len(),
                "confidence": 0.85 // 可以从 deltas 计算平均置信度
            });
            Ok(result)
        }
        Err(e) => {
            eprintln!("Innovation error: {}", e);
            Err(format!("创新分析失败: {}", e))
        }
    }
}

/// 运行创新智能体
#[tauri::command]
async fn run_innovation(
    state: tauri::State<'_, AppState>,
    structured_idea: String,
) -> Result<serde_json::Value, String> {
    println!("Running innovation for structured idea");

    // 模拟创新建议生成
    let mock_innovations = serde_json::json!({
        "status": "completed",
        "deltas": [
            {
                "dimension": "Technology",
                "description": "引入AI自动化功能",
                "impact_score": 8.5,
                "feasibility": 7.0,
                "innovation_level": 8.0
            },
            {
                "dimension": "Business",
                "description": "实施订阅制商业模式",
                "impact_score": 9.0,
                "feasibility": 8.5,
                "innovation_level": 6.5
            }
        ],
        "total_suggestions": 5,
        "confidence": 0.82
    });

    Ok(mock_innovations)
}

/// 运行批评智能体
#[tauri::command]
async fn run_criticism(
    state: tauri::State<'_, AppState>,
    deltas: String,
) -> Result<serde_json::Value, String> {
    println!("Running criticism for deltas");

    // 模拟批评分析
    let mock_criticisms = serde_json::json!({
        "status": "completed",
        "criticisms": [
            {
                "dimension": "Feasibility",
                "severity": 6,
                "description": "AI功能实现复杂度较高，需要大量技术投入",
                "suggestions": ["分阶段实施", "寻找技术合作伙伴"],
                "affected_deltas": ["Technology"]
            },
            {
                "dimension": "Market",
                "severity": 4,
                "description": "订阅模式市场接受度需要验证",
                "suggestions": ["小规模试点", "用户调研"],
                "affected_deltas": ["Business"]
            }
        ],
        "overall_risk_level": "medium",
        "confidence": 0.78
    });

    Ok(mock_criticisms)
}

/// 运行综合智能体
#[tauri::command]
async fn run_synthesis(
    state: tauri::State<'_, AppState>,
    deltas: String,
    criticisms: String,
) -> Result<serde_json::Value, String> {
    println!("Running synthesis for deltas and criticisms");

    // 模拟综合版本生成
    let mock_iteration = serde_json::json!({
        "status": "completed",
        "iteration": {
            "version": "v1.0",
            "summary": "基于AI的订阅制产品优化方案",
            "refined_deltas": [
                {
                    "dimension": "Technology",
                    "description": "分阶段实施AI功能，先实现核心自动化",
                    "adjustments": ["降低初期复杂度", "MVP优先"]
                },
                {
                    "dimension": "Business",
                    "description": "混合定价模式：免费版+高级订阅",
                    "adjustments": ["降低用户门槛", "提供试用期"]
                }
            ],
            "reasoning": "综合考虑了技术可行性和市场风险，提出了更平衡的方案",
            "confidence": 0.85
        }
    });

    Ok(mock_iteration)
}

/// 运行验证智能体
#[tauri::command]
async fn run_verification(
    state: tauri::State<'_, AppState>,
    iteration: String,
) -> Result<serde_json::Value, String> {
    println!("Running verification for iteration");

    // 模拟验证报告
    let mock_verification = serde_json::json!({
        "status": "completed",
        "report": {
            "logical_consistency": {
                "score": 8.5,
                "issues": [],
                "passed": true
            },
            "factual_accuracy": {
                "score": 8.0,
                "issues": ["需要验证AI技术成本估算"],
                "passed": true
            },
            "risk_assessment": {
                "score": 7.5,
                "issues": ["技术实施风险需要更详细的计划"],
                "passed": true
            },
            "overall_passed": true,
            "confidence": 0.8,
            "recommendations": [
                "制定详细的技术实施计划",
                "进行市场调研验证定价策略"
            ]
        }
    });

    Ok(mock_verification)
}

/// 运行总结智能体
#[tauri::command]
async fn run_summarization(
    state: tauri::State<'_, AppState>,
    session_data: String,
) -> Result<serde_json::Value, String> {
    println!("Running summarization for complete session");

    // 模拟完整报告生成
    let mock_summary = serde_json::json!({
        "status": "completed",
        "report": {
            "executive_summary": "通过多智能体协作分析，将原始想法优化为可执行的产品方案",
            "key_insights": [
                "AI功能应分阶段实施以降低风险",
                "混合定价模式能更好平衡收益和用户接受度",
                "需要重点关注技术实施和市场验证"
            ],
            "final_recommendations": [
                "启动MVP开发，重点实现核心AI功能",
                "设计免费版功能吸引初期用户",
                "制定详细的技术路线图和时间表"
            ],
            "next_steps": [
                "技术可行性调研",
                "用户需求验证",
                "原型开发计划"
            ],
            "confidence": 0.83,
            "optimization_quality": "高质量"
        }
    });

    Ok(mock_summary)
}

/// 完整的端到端概念优化流程
#[tauri::command]
async fn run_full_optimization(
    state: tauri::State<'_, AppState>,
    idea_content: String,
) -> Result<serde_json::Value, String> {
    println!("Running full optimization workflow for: {}", idea_content);

    // 模拟完整工作流结果
    let full_result = serde_json::json!({
        "status": "completed",
        "session_id": Uuid::new_v4().to_string(),
        "workflow": {
            "clarification": {
                "questions_asked": 3,
                "slots_filled": 4,
                "confidence": 0.78
            },
            "innovation": {
                "deltas_generated": 8,
                "top_suggestions": 5,
                "confidence": 0.82
            },
            "criticism": {
                "issues_identified": 6,
                "severity_levels": {"high": 1, "medium": 3, "low": 2},
                "confidence": 0.75
            },
            "synthesis": {
                "iterations": 1,
                "final_version": "v1.0",
                "confidence": 0.85
            },
            "verification": {
                "checks_passed": 3,
                "checks_total": 3,
                "overall_passed": true,
                "confidence": 0.8
            },
            "summary": {
                "insights_count": 5,
                "recommendations_count": 4,
                "quality_score": 8.5
            }
        },
        "execution_time": "45.2s",
        "total_confidence": 0.8
    });

    Ok(full_result)
}

/// 获取迭代版本
#[tauri::command]
async fn get_iteration_versions(
    session_id: String,
    _state: tauri::State<'_, AppState>,
) -> Result<Vec<String>, String> {
    println!("Getting iterations for session: {}", session_id);
    Ok(vec![
        "第一版".to_string(),
        "第二版".to_string(),
        "最终版".to_string(),
    ])
}

/// 导出结果
#[tauri::command]
async fn export_result(
    session_id: String,
    format: String,
    _state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    println!(
        "Exporting result for session: {} in format: {}",
        session_id, format
    );
    Ok("导出已完成".to_string())
}

/// 更新模型配置
#[tauri::command]
async fn update_model_config(
    config: String,
    _state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    println!("Updating model config: {}", config);
    Ok(())
}

/// 获取应用配置
#[tauri::command]
async fn get_app_config(state: tauri::State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.read().await;
    Ok(config.clone())
}

/// 更新API密钥
#[tauri::command]
async fn update_api_keys(
    state: tauri::State<'_, AppState>,
    openai_key: Option<String>,
    claude_key: Option<String>,
    deepseek_key: Option<String>,
) -> Result<(), String> {
    println!("Updating API keys");
    let mut config = state.config.write().await;

    if let Some(key) = openai_key {
        config.api_keys.openai_api_key = Some(key);
        println!("Updated OpenAI API key");
    }
    if let Some(key) = claude_key {
        config.api_keys.claude_api_key = Some(key);
        println!("Updated Claude API key");
    }
    if let Some(key) = deepseek_key {
        config.api_keys.deepseek_api_key = Some(key);
        println!("Updated DeepSeek API key");
    }

    Ok(())
}

/// 端到端AI工作流测试
#[tauri::command]
async fn test_full_ai_workflow(
    state: tauri::State<'_, AppState>,
    idea_content: String,
) -> Result<serde_json::Value, String> {
    println!("🚀 开始端到端AI工作流测试: {}", idea_content);

    // 创建 ModelManager 实例
    let model_manager = Arc::new(ModelManager::new(state.config.clone()));

    // 步骤1: 测试澄清智能体
    println!("📝 步骤1: 澄清阶段");
    let clarifier = ClarifierAgent::new(state.config.clone(), model_manager.clone())
        .await
        .map_err(|e| format!("创建澄清代理失败: {}", e))?;

    let idea_seed = IdeaSeed {
        raw_text: idea_content.clone(),
        context_hints: vec!["用户输入".to_string()],
        domain: Some("通用".to_string()),
    };

    let clarification_result = match clarifier.analyze_and_clarify(&idea_seed).await {
        Ok(clarification) => {
            println!("✅ 澄清阶段成功完成");
            serde_json::json!({
                "status": "success",
                "questions_count": clarification.qa_pairs.len(),
                "confidence": clarification.confidence,
                "structured_idea": clarification.structured_idea
            })
        }
        Err(e) => {
            println!("❌ 澄清阶段失败: {}", e);
            serde_json::json!({
                "status": "error",
                "error": format!("澄清失败: {}", e),
                "fallback_used": true
            })
        }
    };

    // 步骤2: 如果澄清成功，测试创新智能体
    let innovation_result = if clarification_result["status"] == "success" {
        println!("💡 步骤2: 创新阶段");

        // 使用澄清结果中的结构化想法，或创建默认的
        let structured_idea =
            if let Some(idea) = clarification_result["structured_idea"].as_object() {
                StructuredIdea {
                    target: Some(
                        idea.get("target")
                            .and_then(|v| v.as_str())
                            .unwrap_or("未指定目标")
                            .to_string(),
                    ),
                    stakeholders: vec!["用户".to_string()],
                    constraints: {
                        let mut map = HashMap::new();
                        map.insert("时间".to_string(), "限制".to_string());
                        map
                    },
                    deliverables: vec!["可行性方案".to_string()],
                    success_metrics: vec!["用户满意度".to_string()],
                    risks_assumptions: vec!["技术可行性".to_string()],
                }
            } else {
                StructuredIdea {
                    target: Some("改进现有系统".to_string()),
                    stakeholders: vec!["终端用户".to_string(), "开发团队".to_string()],
                    constraints: {
                        let mut map = HashMap::new();
                        map.insert("预算".to_string(), "限制".to_string());
                        map.insert("时间".to_string(), "限制".to_string());
                        map
                    },
                    deliverables: vec!["原型系统".to_string(), "技术文档".to_string()],
                    success_metrics: vec!["性能提升".to_string(), "用户满意度".to_string()],
                    risks_assumptions: vec!["技术栈可用".to_string(), "团队技能足够".to_string()],
                }
            };

        let innovator = InnovatorAgent::new(state.config.clone(), model_manager.clone())
            .await
            .map_err(|e| format!("创建创新代理失败: {}", e))?;

        match innovator.generate_deltas(&structured_idea).await {
            Ok(deltas) => {
                println!("✅ 创新阶段成功完成，生成{}个改进建议", deltas.len());
                serde_json::json!({
                    "status": "success",
                    "deltas_count": deltas.len(),
                    "top_suggestions": deltas.iter().take(3).map(|d| &d.content).collect::<Vec<_>>()
                })
            }
            Err(e) => {
                println!("❌ 创新阶段失败: {}", e);
                serde_json::json!({
                    "status": "error",
                    "error": format!("创新失败: {}", e)
                })
            }
        }
    } else {
        serde_json::json!({
            "status": "skipped",
            "reason": "clarification_failed"
        })
    };

    // 返回完整测试结果
    let final_result = serde_json::json!({
        "test_status": "completed",
        "idea_content": idea_content,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "results": {
            "clarification": clarification_result,
            "innovation": innovation_result
        },
        "summary": {
            "total_steps": 2,
            "successful_steps": if clarification_result["status"] == "success" && innovation_result["status"] == "success" { 2 } else if clarification_result["status"] == "success" { 1 } else { 0 },
            "ai_integration_working": clarification_result["status"] == "success" || innovation_result["status"] == "success"
        }
    });

    println!("🎉 端到端AI工作流测试完成");
    Ok(final_result)
}

/// 测试AI连接
#[tauri::command]
async fn test_ai_connection(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    println!("Testing AI connection");

    let config = state.config.read().await;

    // 简单检查是否有API密钥配置
    let has_openai = config.api_keys.openai_api_key.is_some();
    let has_claude = config.api_keys.claude_api_key.is_some();
    let has_deepseek = config.api_keys.deepseek_api_key.is_some();

    if has_openai || has_claude || has_deepseek {
        println!("AI connection test successful - API keys configured");
        Ok(true)
    } else {
        println!("AI connection test failed - no API keys configured");
        Ok(false)
    }
}

async fn setup_app() -> AppState {
    println!("Initializing MAgent application...");

    // 初始化配置
    let config = Arc::new(RwLock::new(AppConfig::new()));
    println!("Configuration initialized");

    // 创建事件总线
    let (event_tx, _event_rx) = mpsc::unbounded_channel::<SystemEvent>();

    // 初始化存储
    let storage = Arc::new(
        DataStore::new()
            .await
            .expect("Failed to initialize storage"),
    );
    println!("Storage initialized");

    // 创建Agent运行时
    let agent_runtime = Arc::new(
        agents::AgentRuntime::new(config.clone(), storage.clone(), event_tx.clone())
            .await
            .expect("Failed to initialize agent runtime"),
    );
    println!("Agent runtime initialized");

    let app_state = AppState {
        agent_runtime,
        config,
        event_bus: event_tx,
        storage,
    };

    println!("MAgent application setup completed");
    app_state
}

#[tokio::main]
async fn main() {
    let app_state = setup_app().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            start_concept_optimization,
            run_clarification_ai,
            run_innovation,
            run_innovation_ai,
            run_criticism,
            run_synthesis,
            run_verification,
            run_summarization,
            run_full_optimization,
            get_iteration_versions,
            export_result,
            update_model_config,
            get_app_config,
            update_api_keys,
            test_full_ai_workflow,
            test_ai_connection
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
