use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rstar::AABB;

use crate::ServerCtx;
use common_data::{DamageType, RoadDamage, RoaddamageAdditionalInfo};

pub async fn root() -> impl IntoResponse {
    "Hewwo wowd"
}

pub async fn get_points_in_rect(
    State(ctx): State<Arc<ServerCtx>>,
    Path((lon1, lat1, lon2, lat2)): Path<(f64, f64, f64, f64)>,
    Query(params): Query<HashMap<String, u8>>,
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
        Ok(Some(v)) => Ok(Json(v)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
