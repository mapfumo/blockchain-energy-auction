use sqlx::{PgPool, PgConnection, Connection};
use std::env;
use anyhow::Result;

pub struct DatabaseConnection {
    pub pool: PgPool,
}

pub async fn create_pool() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://energy_user:energy_pass@localhost:5432/energy_trading".to_string());
    
    let pool = PgPool::connect(&database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self> {
        let pool = create_pool().await?;
        Ok(Self { pool })
    }
}

pub async fn create_connection() -> Result<PgConnection> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://energy_user:energy_pass@localhost:5432/energy_trading".to_string());
    
    let conn = PgConnection::connect(&database_url).await?;
    Ok(conn)
}
