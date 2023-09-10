pub mod analysis;

use std::{collections::HashMap, sync::Arc};

use axum::{
    body::StreamBody,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use rstar::AABB;
use tokio::io::AsyncReadExt;

use crate::ServerCtx;
use common_data::{DamageType, RoadDamage, RoaddamageAdditionalInfo};

pub async fn root() -> impl IntoResponse {
    "Hewwo wowd"
}

pub async fn get_points_in_rect(
    State(ctx): State<Arc<ServerCtx>>,
    Path((lon1, lat1, lon2, lat2)): Path<(f64, f64, f64, f64)>,
    Query(params): Query<HashMap<String, u16>>,
) -> Json<Vec<RoadDamage>> {
    let p1 = [lon1, lat1];
    let p2 = [lon2, lat2];
    let aabb = AABB::from_corners(p1, p2);
    let filter: Box<dyn Fn(&RoadDamage) -> bool> = match params.get("filter-damage-type") {
        Some(v) => {
            Box::new(|el: &RoadDamage| (el.damage_type & DamageType::from_bits_truncate(*v)).into())
        }
        None => Box::new(|_el| true),
    };

    Json(
        ctx.tree
            .lock()
            .unwrap()
            .locate_in_envelope(&aabb)
            .filter(|el| (filter)(el))
            .map(|el| el.clone())
            .collect::<Vec<_>>(),
    )
}

pub async fn get_additional_info_for_point(
    State(ctx): State<Arc<ServerCtx>>,
    Path(id): Path<i64>,
) -> Result<Json<RoaddamageAdditionalInfo>, StatusCode> {
    match ctx.db.get_additional_info(id).await {
        Ok(v) => Ok(Json(v)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
pub async fn fetch_image(State(ctx): State<Arc<ServerCtx>>, Path(id): Path<i64>) -> Response {
    match ctx.db.get_basic_info(id).await {
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        Ok(Some(v)) => {
            let path = format!(
                "{}/{}_{}.jpg",
                std::env::var("IMAGE_DIR").unwrap(),
                v.longitude,
                v.latitude
            );
            let mut file = match tokio::fs::File::open(&path).await {
                Ok(file) => file,
                Err(err) => {
                    return (StatusCode::NOT_FOUND, format!("File not found: {}", err))
                        .into_response()
                }
            };

            let mut data = vec![];
            file.read_to_end(&mut data).await.unwrap();

            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", HeaderValue::from_static("image/jpeg"));

            (headers, data).into_response()
        }
    }
}
