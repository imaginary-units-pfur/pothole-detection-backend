use sqlx::sqlite::SqlitePool;
use std::env;
use tracing::{error, info, warn};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = SqlitePool::connect(&database_url)
            .await
            .expect("Could not connect to the database at {database_url}");
        Self { pool }
    }
}
