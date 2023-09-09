use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

use common_data::{RoadDamage, RoaddamageAdditionalInfo};

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Self {
        let _ = dotenv();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = SqlitePool::connect(&database_url).await.expect(&format!(
            "Could not connect to the database at {database_url}"
        ));
        sqlx::migrate!().run(&pool).await.unwrap();
        Self { pool }
    }

    pub async fn get_all_records(&self) -> sqlx::Result<Vec<RoadDamage>> {
        Ok(sqlx::query_as!(
            RoadDamage,
            r#"
        SELECT id, damage_type, latitude, longitude
        FROM road_damage
        "#
        )
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_additional_info(
        &self,
        id: i64,
    ) -> sqlx::Result<Option<RoaddamageAdditionalInfo>> {
        Ok(sqlx::query_as!(
            RoaddamageAdditionalInfo,
            r#"
        SELECT file_path
        FROM road_damage
        WHERE id = ?
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }
}
