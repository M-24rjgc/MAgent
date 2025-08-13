use super::*;
use std::fs;
use std::path::Path;

// Import all agents
use crate::agents::{
    clarifier::ClarifierAgent,
    innovator::InnovatorAgent,
    critic::CriticAgent,
    synthesizer::SynthesizerAgent,
    verifier::VerifierAgent,
    summarizer::SummarizerAgent,
};

// Import data structures
use crate::core::IdeaSeed;
use crate::agents::AgentResult;
use crate::config::AppConfig;
use crate::storage::DataStore;
use std::sync::Arc;
use tokio::sync::RwLock;

// Basic agent construction test
#[tokio::test]
async fn test_agent_construction() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing agent construction...");

    // Create test config
    let config = Arc::new(RwLock::new(AppConfig::default()));
    
    // Create test storage for verifier
    let storage = Arc::new(DataStore::new("test.db").await?);

    // Test each agent can be constructed
    let _clarifier = ClarifierAgent::new(config.clone()).await?;
    println!("ClarifierAgent construction: OK");

    let _innovator = InnovatorAgent::new(config.clone()).await?;
    println!("InnovatorAgent construction: OK");

    let _critic = CriticAgent::new(config.clone()).await?;
    println!("CriticAgent construction: OK");

    let _synthesizer = SynthesizerAgent::new(config.clone()).await?;
    println!("SynthesizerAgent construction: OK");

    let _verifier = VerifierAgent::new(config.clone(), storage.clone()).await?;
    println!("VerifierAgent construction: OK");

    let _summarizer = SummarizerAgent::new(config.clone()).await?;
    println!("SummarizerAgent construction: OK");

    println!("All agents constructed successfully!");
    Ok(())
}

// Simple workflow test
#[tokio::test]
async fn test_simple_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing basic 6-agent workflow...");

    // Create test config and storage
    let config = Arc::new(RwLock::new(AppConfig::default()));
    let storage = Arc::new(DataStore::new("test_workflow.db").await?);

    // Create test data
    let idea = IdeaSeed {
        id: "test-001".to_string(),
        session_id: "session-001".to_string(),
        raw_text: "Build an AI-powered productivity app".to_string(),
        complexity_level: 3,
        domain: "Technology".to_string(),
        timestamp: chrono::Utc::now(),
    };

    println!("Input idea: {}", idea.raw_text);

    // 1. Clarifier Agent
    let clarifier = ClarifierAgent::new(config.clone()).await?;
    let clarification_result = clarifier.process(&idea).await?;
    let clarification = match clarification_result {
        AgentResult::Clarification(c) => {
            println!("Clarifier: {} questions generated", c.qa_pairs.len());
            c
        },
        _ => panic!("Expected Clarification result"),
    };

    // 2. Innovator Agent  
    let innovator = InnovatorAgent::new(config.clone()).await?;
    let innovation_result = innovator.process(&clarification).await?;
    let innovation_deltas = match innovation_result {
        AgentResult::Innovation(deltas) => {
            println!("Innovator: {} innovations generated", deltas.len());
            deltas
        },
        _ => panic!("Expected Innovation result"),
    };

    // 3. Critic Agent
    let critic = CriticAgent::new(config.clone()).await?;
    let criticism_result = critic.process(&innovation_deltas).await?;
    let criticisms = match criticism_result {
        AgentResult::Criticism(crits) => {
            println!("Critic: {} criticisms generated", crits.len());
            crits
        },
        _ => panic!("Expected Criticism result"),
    };

    // 4. Synthesizer Agent
    let synthesizer = SynthesizerAgent::new(config.clone()).await?;
    let synthesis_result = synthesizer.synthesize(
        clarification.clone(),
        innovation_deltas.clone(),
        criticisms.clone(),
    ).await?;
    let synthesis = match synthesis_result {
        AgentResult::Synthesis(iter) => {
            println!("Synthesizer: version {} generated", iter.version);
            iter
        },
        _ => panic!("Expected Synthesis result"),
    };

    // 5. Verifier Agent
    let verifier = VerifierAgent::new(config.clone(), storage.clone()).await?;
    let verification_result = verifier.verify(&synthesis).await?;
    let verification_report = match verification_result {
        AgentResult::Verification(report) => {
            println!("Verifier: validation status = {}", report.passed);
            report
        },
        _ => panic!("Expected Verification result"),
    };

    // 6. Summarizer Agent
    let summarizer = SummarizerAgent::new(config.clone()).await?;
    let summary_result = summarizer.generate_summary(
        &idea,
        &clarification,
        &innovation_deltas,
        &criticisms,
        &synthesis,
        &verification_report,
    ).await?;
    let final_summary = match summary_result {
        AgentResult::Summary(summary) => {
            println!("Summarizer: {} chars report generated", summary.len());
            summary
        },
        _ => panic!("Expected Summary result"),
    };

    // Verify workflow completion
    assert!(!clarification.qa_pairs.is_empty());
    assert!(!innovation_deltas.is_empty());
    assert!(!criticisms.is_empty());
    assert!(synthesis.version > 0);
    assert!(!final_summary.is_empty());

    println!("Complete workflow test passed!");
    println!("  - Clarifier: {} questions", clarification.qa_pairs.len());
    println!("  - Innovator: {} innovations", innovation_deltas.len());
    println!("  - Critic: {} criticisms", criticisms.len());
    println!("  - Synthesizer: v{} (confidence: {:.1}%)", 
        synthesis.version, synthesis.confidence_score);
    println!("  - Verifier: passed = {}", verification_report.passed);
    println!("  - Summarizer: {} chars", final_summary.len());

    Ok(())
}

// Error handling test
#[tokio::test]
async fn test_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing error handling...");

    // Create test config
    let config = Arc::new(RwLock::new(AppConfig::default()));

    // Test empty input
    let empty_idea = IdeaSeed {
        id: "empty-test".to_string(),
        session_id: "session-empty".to_string(),
        raw_text: "".to_string(),
        complexity_level: 0,
        domain: "Test".to_string(),
        timestamp: chrono::Utc::now(),
    };

    let clarifier = ClarifierAgent::new(config).await?;
    let result = clarifier.process(&empty_idea).await;

    match result {
        Ok(_) => println!("Empty input handled gracefully"),
        Err(e) => println!("Expected error handling: {}", e),
    }

    println!("Error handling test completed");
    Ok(())
}
