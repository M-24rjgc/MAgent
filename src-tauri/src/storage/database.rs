use anyhow::Result;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn init_database() -> Result<SqlitePool> {
    // Use in-memory database for now to avoid file permission issues
    let connection_string = "sqlite::memory:";
    
    // Create database pool
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(connection_string)
        .await?;

    // Run migrations
    create_tables(&pool).await?;

    Ok(pool)
}

async fn create_tables(pool: &SqlitePool) -> Result<()> {
    // Create sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            idea_seed TEXT NOT NULL,
            state TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create iterations table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS iterations (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            version_number INTEGER NOT NULL,
            summary TEXT NOT NULL,
            deltas TEXT NOT NULL,
            rationale TEXT NOT NULL,
            scores TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions (id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create clarifications table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS clarifications (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            qa_pairs TEXT NOT NULL,
            open_slots TEXT NOT NULL,
            confidence REAL NOT NULL,
            structured_idea TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions (id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create verification_reports table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS verification_reports (
            id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            logic_checks TEXT NOT NULL,
            fact_checks TEXT NOT NULL,
            risks TEXT NOT NULL,
            passed BOOLEAN NOT NULL,
            confidence REAL NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions (id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create knowledge_base table for caching
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS knowledge_base (
            id TEXT PRIMARY KEY,
            source_type TEXT NOT NULL,
            source_id TEXT NOT NULL,
            content TEXT NOT NULL,
            metadata TEXT,
            embedding BLOB,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}
