use dotenvy::dotenv;
use sqlx::mysql::MySqlPool;
use std::{env, path::Path};

use common_data::{DamageType, RoadDamage, RoaddamageAdditionalInfo};

#[derive(Clone)]
pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub async fn new() -> Self {
        let _ = dotenv();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPool::connect(&database_url).await.expect(&format!(
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

    pub async fn get_basic_info(&self, id: i64) -> sqlx::Result<Option<RoadDamage>> {
        Ok(sqlx::query_as!(
            RoadDamage,
            r#"
            SELECT id, damage_type, latitude, longitude
            FROM road_damage
        WHERE id = ?
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn get_additional_info(&self, id: i64) -> sqlx::Result<RoaddamageAdditionalInfo> {
        Ok({
            let answer = sqlx::query!(
                r#"
        SELECT file_path, top_certainty, top_type
        FROM road_damage
        WHERE id = ?
        "#,
                id
            )
            .fetch_one(&self.pool)
            .await?;
            RoaddamageAdditionalInfo {
                file_path: std::path::PathBuf::from(answer.file_path),
                top_certainty: answer.top_certainty.unwrap(),
                top_type: answer.top_type.unwrap(),
            }
        })
    }

    pub async fn insert_new(
        &self,
        damage_type: u16,
        file_path: std::path::PathBuf,
        latitude: f64,
        longitude: f64,
        top_certainty: f64,
        top_type: &str,
    ) -> sqlx::Result<i64> {
        let mut trans = self.pool.begin().await?;
        let file_path = file_path.to_str().unwrap();
        sqlx::query!(
            r#"
        INSERT INTO road_damage (damage_type, file_path, longitude, latitude, top_certainty, top_type)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
            damage_type,
            file_path,
            longitude,
            latitude,
            top_certainty,
            top_type
        ).execute(&mut *trans).await;
        let id = sqlx::query!("SELECT LAST_INSERT_ID() as id;")
            .fetch_one(&mut *trans)
            .await?
            .id;
        trans.commit().await?;
        Ok(id as i64)
    }

    #[cfg(not(feature = "mock_data"))]
    pub async fn create_demo_data(&self) -> sqlx::Result<()> {
        // Never fabricate data in release mode
        Ok(())
    }

    #[cfg(feature = "mock_data")]
    pub async fn create_mock_data(&self) -> sqlx::Result<()> {
        // use common_data::DamageType;
        // use rand::prelude::*;
        // use sqlx::query;

        // if let Some(_) = sqlx::query!("SELECT * FROM road_damage LIMIT 1")
        //     .fetch_optional(&self.pool)
        //     .await?
        // {
        //     // Data already exists, do not insert mock data.
        //     return Ok(());
        // }

        // // These correspond to the inner Moscow area
        // let bounds_lat = 55.40..=55.70;
        // let bounds_lng = 37.30..=37.70;

        // let point_count = 1000;
        // eprintln!("!!! Generating {point_count} random points where lat={bounds_lat:?} and lng={bounds_lng:?}");

        // let mut rng = rand::thread_rng();

        // let mut tx = self.pool.begin().await?;

        // for _ in 0..point_count {
        //     let lat = rng.gen_range(bounds_lat.clone());
        //     let lng = rng.gen_range(bounds_lng.clone());
        //     let file_path = "";
        //     let damage_type = [
        //         DamageType::Crack,
        //         DamageType::Patch,
        //         DamageType::Pothole,
        //         DamageType::Other,
        //     ]
        //     .choose(&mut rng)
        //     .unwrap()
        //     .bits();
        //     query!("INSERT INTO road_damage (damage_type, file_path, longitude, latitude) VALUES (?,?,?,?);", damage_type, file_path, lng, lat)
        //         .execute(&mut *tx)
        //         .await?;
        // }

        // tx.commit().await?;

        Ok(())
    }
}
