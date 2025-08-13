// src-tauri/src/tests/integration_test_simple.rs
// 简化的集成测试 - 验证6个Agent的基础协作工作流 (英文注释版本)

use anyhow::Result;
use std::sync::Arc;
use tokio;
use uuid::Uuid;

use crate::agents::{
    clarifier::ClarifierAgent, critic::CriticAgent, innovator::InnovatorAgent,
    summarizer::SummarizerAgent, synthesizer::SynthesizerAgent, verifier::VerifierAgent, Agent,
    AgentContext, AgentResult,
};
use crate::config::AppConfig;
use crate::core::data_structures::IdeaSeed;
use crate::models::manager::ModelManager;
use crate::storage::DataStore;

/// Simple integration test for 6-agent workflow
#[tokio::test]
async fn test_full_agent_workflow() -> Result<()> {
    println!("🚀 Starting integration test: Full 6-Agent workflow");

    // 1. Setup test environment
    let config = Arc::new(tokio::sync::RwLock::new(AppConfig::default()));
    let model_manager: Arc<ModelManager> = Arc::new(ModelManager::new(config.clone()));
    let data_store = Arc::new(DataStore::new().await?);

    // 2. Create test idea
    let idea_seed = IdeaSeed {
        raw_text: "I want to build an AI assistant to help students improve learning efficiency"
            .to_string(),
        context_hints: vec![
            "Education Technology".to_string(),
            "AI Assistant".to_string(),
        ],
        domain: Some("Education".to_string()),
    };

    let session_id = Uuid::new_v4();
    println!("💡 Test idea: {}", idea_seed.raw_text);

    // 3. Test Clarifier Agent
    println!("\n📋 Phase 1: Testing Clarifier Agent...");

    let clarifier_context = AgentContext {
        session_id,
        current_version: None,
        clarification: None,
        previous_versions: vec![],
        knowledge_base: vec![],
        previous_results: vec![],
    };

    let clarifier = ClarifierAgent::new(config.clone(), model_manager.clone()).await?;
    let clarifier_result = clarifier.execute(clarifier_context).await?;

    let clarification = match clarifier_result {
        AgentResult::Clarification(c) => {
            println!(
                "✅ Clarifier completed: {} questions generated",
                c.qa_pairs.len()
            );
            c
        }
        _ => panic!("Expected Clarification result"),
    };

    // 4. Test Innovator Agent
    println!("\n💡 Phase 2: Testing Innovator Agent...");

    let innovator_context = AgentContext {
        session_id,
        current_version: None,
        clarification: Some(clarification.clone()),
        previous_versions: vec![],
        knowledge_base: vec![],
        previous_results: vec![AgentResult::Clarification(clarification.clone())],
    };

    let innovator = InnovatorAgent::new(config.clone(), model_manager.clone()).await?;
    let innovator_result = innovator.execute(innovator_context).await?;

    let innovation_deltas = match innovator_result {
        AgentResult::Innovation(deltas) => {
            println!(
                "✅ Innovator completed: {} innovation suggestions generated",
                deltas.len()
            );
            deltas
        }
        _ => panic!("Expected Innovation result"),
    };

    // 5. Test Critic Agent
    println!("\n🔍 Phase 3: Testing Critic Agent...");

    let critic_context = AgentContext {
        session_id,
        current_version: None,
        clarification: Some(clarification.clone()),
        previous_versions: vec![],
        knowledge_base: vec![],
        previous_results: vec![
            AgentResult::Clarification(clarification.clone()),
            AgentResult::Innovation(innovation_deltas.clone()),
        ],
    };

    let critic = CriticAgent::new(config.clone(), model_manager.clone()).await?;
    let critic_result = critic.execute(critic_context).await?;

    let criticisms = match critic_result {
        AgentResult::Criticism(crits) => {
            println!("✅ Critic completed: {} criticisms generated", crits.len());
            crits
        }
        _ => panic!("Expected Criticism result"),
    };

    // 6. Test Synthesizer Agent
    println!("\n🔄 Phase 4: Testing Synthesizer Agent...");

    let synthesizer_context = AgentContext {
        session_id,
        current_version: None,
        clarification: Some(clarification.clone()),
        previous_versions: vec![],
        knowledge_base: vec![],
        previous_results: vec![
            AgentResult::Clarification(clarification.clone()),
            AgentResult::Innovation(innovation_deltas.clone()),
            AgentResult::Criticism(criticisms.clone()),
        ],
    };

    let synthesizer = SynthesizerAgent::new(config.clone(), model_manager.clone()).await?;
    let synthesizer_result = synthesizer.execute(synthesizer_context).await?;

    let iteration_version = match synthesizer_result {
        AgentResult::Synthesis(iter) => {
            println!(
                "✅ Synthesizer completed: version v{} generated",
                iter.version_number
            );
            iter
        }
        _ => panic!("Expected Synthesis result"),
    };

    // 7. Test Verifier Agent
    println!("\n🔍 Phase 5: Testing Verifier Agent...");

    let verifier_context = AgentContext {
        session_id,
        current_version: Some(iteration_version.clone()),
        clarification: Some(clarification.clone()),
        previous_versions: vec![],
        knowledge_base: vec![],
        previous_results: vec![
            AgentResult::Clarification(clarification.clone()),
            AgentResult::Innovation(innovation_deltas.clone()),
            AgentResult::Criticism(criticisms.clone()),
            AgentResult::Synthesis(iteration_version.clone()),
        ],
    };

    let verifier =
        VerifierAgent::new(config.clone(), data_store.clone(), model_manager.clone()).await?;
    let verifier_result = verifier.execute(verifier_context).await?;

    let verification_report = match verifier_result {
        AgentResult::Verification(report) => {
            println!(
                "✅ Verifier completed: verification status: {}",
                report.passed
            );
            report
        }
        _ => panic!("Expected Verification result"),
    };

    // 8. Test Summarizer Agent
    println!("\n📊 Phase 6: Testing Summarizer Agent...");

    let summarizer_context = AgentContext {
        session_id,
        current_version: Some(iteration_version.clone()),
        clarification: Some(clarification.clone()),
        previous_versions: vec![],
        knowledge_base: vec![],
        previous_results: vec![
            AgentResult::Clarification(clarification.clone()),
            AgentResult::Innovation(innovation_deltas.clone()),
            AgentResult::Criticism(criticisms.clone()),
            AgentResult::Synthesis(iteration_version.clone()),
            AgentResult::Verification(verification_report.clone()),
        ],
    };

    let summarizer = SummarizerAgent::new(config.clone(), model_manager.clone()).await?;
    let summarizer_result = summarizer.execute(summarizer_context).await?;

    let final_summary = match summarizer_result {
        AgentResult::Summary(summary) => {
            println!(
                "✅ Summarizer completed: {} character report generated",
                summary.len()
            );
            summary
        }
        _ => panic!("Expected Summary result"),
    };

    // 9. Validate workflow completeness
    println!("\n🏁 Workflow validation...");

    // Validate data flow consistency
    assert!(
        !clarification.qa_pairs.is_empty(),
        "Clarification should generate questions"
    );
    assert!(
        !innovation_deltas.is_empty(),
        "Innovation should generate suggestions"
    );
    // Note: Critic may not generate criticisms if the idea is good - this is expected behavior
    println!(
        "   - Critic criticisms: {} (OK if 0 - means idea is solid)",
        criticisms.len()
    );
    assert!(
        iteration_version.version_number > 0,
        "Synthesis should generate versions"
    );
    assert!(!final_summary.is_empty(), "Summary should generate report");

    println!("✅ Integration test completed! All 6 Agents successfully collaborated");
    println!("📈 Test results summary:");
    println!("   - Clarifier: {} questions", clarification.qa_pairs.len());
    println!("   - Innovator: {} innovations", innovation_deltas.len());
    println!("   - Critic: {} criticisms", criticisms.len());
    println!(
        "   - Synthesizer: v{}, coherence {:.1}%",
        iteration_version.version_number,
        iteration_version.scores.coherence * 100.0
    );
    println!("   - Verifier: status {}", verification_report.passed);
    println!("   - Summarizer: {} character report", final_summary.len());

    Ok(())
}

/// Test Agent construction and basic capabilities
#[tokio::test]
async fn test_agent_construction() -> Result<()> {
    println!("🔧 Starting test: Agent construction and basic capabilities");

    let config = Arc::new(tokio::sync::RwLock::new(AppConfig::default()));
    let model_manager: Arc<ModelManager> = Arc::new(ModelManager::new(config.clone()));
    let data_store = Arc::new(DataStore::new().await?);

    // Test each agent construction
    let _clarifier = ClarifierAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ ClarifierAgent construction successful");

    let _innovator = InnovatorAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ InnovatorAgent construction successful");

    let _critic = CriticAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ CriticAgent construction successful");

    let _synthesizer = SynthesizerAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ SynthesizerAgent construction successful");

    let _verifier =
        VerifierAgent::new(config.clone(), data_store.clone(), model_manager.clone()).await?;
    println!("✅ VerifierAgent construction successful");

    let _summarizer = SummarizerAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ SummarizerAgent construction successful");

    println!("✅ Agent construction test completed!");

    Ok(())
}
