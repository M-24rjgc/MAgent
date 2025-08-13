use std::sync::Arc;
use tokio::sync::RwLock;

// Import all agents and traits
use crate::agents::{
    clarifier::ClarifierAgent, critic::CriticAgent, innovator::InnovatorAgent,
    summarizer::SummarizerAgent, synthesizer::SynthesizerAgent, verifier::VerifierAgent,
};

// Import configs
use crate::config::AppConfig;
use crate::models::manager::ModelManager;
use crate::storage::DataStore;

// Very basic agent construction test
#[tokio::test]
async fn test_agent_construction_only() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing agent construction...");

    // Create test config
    let config = Arc::new(RwLock::new(AppConfig::default()));

    // Create model manager
    let model_manager: Arc<ModelManager> = Arc::new(ModelManager::new(config.clone()));

    // Create test storage for verifier
    let storage = Arc::new(DataStore::new().await?);

    // Test each agent can be constructed
    let _clarifier = ClarifierAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ ClarifierAgent construction: OK");

    let _innovator = InnovatorAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ InnovatorAgent construction: OK");

    let _critic = CriticAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ CriticAgent construction: OK");

    let _synthesizer = SynthesizerAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ SynthesizerAgent construction: OK");

    let _verifier =
        VerifierAgent::new(config.clone(), storage.clone(), model_manager.clone()).await?;
    println!("✅ VerifierAgent construction: OK");

    let _summarizer = SummarizerAgent::new(config.clone(), model_manager.clone()).await?;
    println!("✅ SummarizerAgent construction: OK");

    println!("🎉 All 6 agents constructed successfully!");
    Ok(())
}
