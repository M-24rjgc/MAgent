// 抑制开发期间的未使用代码警告
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use anyhow::Result;
use sqlx::{SqlitePool, Row};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod database;
pub mod vector_store;
pub mod cache;

use crate::core::data_structures::*;

/// 数据存储层
pub struct DataStore {
    db_pool: SqlitePool,
    // vector_store: Arc<dyn VectorStore>,
    cache: Arc<RwLock<cache::MemoryCache>>,
}

impl DataStore {
    pub async fn new() -> Result<Self> {
        // Initialize SQLite database
        let db_pool = database::init_database().await?;
        
        // Initialize cache
        let cache = Arc::new(RwLock::new(cache::MemoryCache::new()));

        // TODO: Initialize vector store
        // let vector_store = Arc::new(vector_store::QdrantStore::new().await?);

        Ok(Self {
            db_pool,
            // vector_store,
            cache,
        })
    }

    // Session management
    pub async fn create_session(&self, idea_seed: &IdeaSeed) -> Result<uuid::Uuid> {
        let session_id = uuid::Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO sessions (id, idea_seed, state, created_at, updated_at) VALUES (?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind(session_id.to_string())
        .bind(serde_json::to_string(idea_seed)?)
        .bind("Initializing")
        .execute(&self.db_pool)
        .await?;

        Ok(session_id)
    }

    pub async fn get_session(&self, session_id: uuid::Uuid) -> Result<Option<crate::core::OptimizationSession>> {
        let row = sqlx::query(
            "SELECT id, idea_seed, state, created_at, updated_at FROM sessions WHERE id = ?"
        )
        .bind(session_id.to_string())
        .fetch_optional(&self.db_pool)
        .await?;

        if let Some(row) = row {
            let idea_seed: IdeaSeed = serde_json::from_str(row.try_get("idea_seed")?)?;
            let state_str: String = row.try_get("state")?;
            let state = match state_str.as_str() {
                "Initializing" => crate::core::SessionState::Initializing,
                "Clarifying" => crate::core::SessionState::Clarifying,
                "Clarified" => crate::core::SessionState::Clarified,
                "Done" => crate::core::SessionState::Done,
                _ => crate::core::SessionState::Error("Unknown state".to_string()),
            };
            
            let created_at: String = row.try_get("created_at")?;
            let updated_at: String = row.try_get("updated_at")?;

            Ok(Some(crate::core::OptimizationSession {
                id: session_id,
                idea_seed,
                current_state: state,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.into(),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)?.into(),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_session_state(&self, session_id: uuid::Uuid, state: &crate::core::SessionState) -> Result<()> {
        let state_str = match state {
            crate::core::SessionState::Initializing => "Initializing",
            crate::core::SessionState::Clarifying => "Clarifying", 
            crate::core::SessionState::Clarified => "Clarified",
            crate::core::SessionState::AdvIterating(n) => "AdvIterating",
            crate::core::SessionState::Verified => "Verified",
            crate::core::SessionState::Formatting => "Formatting",
            crate::core::SessionState::Done => "Done",
            crate::core::SessionState::Error(_) => "Error",
        };

        sqlx::query(
            "UPDATE sessions SET state = ?, updated_at = datetime('now') WHERE id = ?"
        )
        .bind(state_str)
        .bind(session_id.to_string())
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    // Iteration management
    pub async fn save_iteration(&self, session_id: uuid::Uuid, iteration: &IterationVersion) -> Result<()> {
        sqlx::query(
            "INSERT INTO iterations (id, session_id, version_number, summary, deltas, rationale, scores, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(iteration.id.to_string())
        .bind(session_id.to_string())
        .bind(iteration.version_number as i64)
        .bind(&iteration.summary)
        .bind(serde_json::to_string(&iteration.deltas)?)
        .bind(&iteration.rationale)
        .bind(serde_json::to_string(&iteration.scores)?)
        .bind(iteration.created_at.to_rfc3339())
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn get_iterations(&self, session_id: uuid::Uuid) -> Result<Vec<IterationVersion>> {
        let rows = sqlx::query(
            "SELECT id, version_number, summary, deltas, rationale, scores, created_at FROM iterations WHERE session_id = ? ORDER BY version_number"
        )
        .bind(session_id.to_string())
        .fetch_all(&self.db_pool)
        .await?;

        let mut iterations = Vec::new();
        for row in rows {
            let id: String = row.try_get("id")?;
            let version_number: i64 = row.try_get("version_number")?;
            let summary: String = row.try_get("summary")?;
            let deltas: String = row.try_get("deltas")?;
            let rationale: String = row.try_get("rationale")?;
            let scores: String = row.try_get("scores")?;
            let created_at: String = row.try_get("created_at")?;
            
            let iteration = IterationVersion {
                id: uuid::Uuid::parse_str(&id)?,
                version_number: version_number as u32,
                summary,
                deltas: serde_json::from_str(&deltas)?,
                rationale,
                scores: serde_json::from_str(&scores)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.into(),
            };
            iterations.push(iteration);
        }

        Ok(iterations)
    }

    // Knowledge retrieval (placeholder)
    pub async fn retrieve_knowledge(&self, query: &str, limit: usize) -> Result<Vec<Evidence>> {
        // TODO: Implement actual knowledge retrieval using vector store
        tracing::info!("Retrieving knowledge for query: {} (limit: {})", query, limit);
        
        // Return placeholder evidence
        Ok(vec![
            Evidence {
                source_id: "placeholder-1".to_string(),
                snippet: format!("相关信息片段，查询：{}", query),
                relevance: 0.8,
                url: None,
            }
        ])
    }
}
