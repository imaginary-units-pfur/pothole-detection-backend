use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

use common_data::RoadDamage;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Self {
        let _ = dotenv();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = SqlitePool::connect(&database_url)
            .await
            .expect("Could not connect to the database at {database_url}");
        sqlx::migrate!().run(&pool).await.unwrap();
        Self { pool }
    }

    pub async fn get_all_records(&self) -> Vec<RoadDamage> {
        sqlx::query_as!(
            RoadDamage,
            r#"
        SELECT damage_type, file_path, latitude, longitude
        FROM road_damage
        "#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap()
    }
}
