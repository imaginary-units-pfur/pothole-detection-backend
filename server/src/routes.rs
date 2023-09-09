use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use rstar::AABB;

use crate::ServerCtx;
use common_data::DamageType;

pub async fn root() -> impl IntoResponse {
    "Hewwo wowd"
}

pub async fn get_points_in_rect(
    State(ctx): State<Arc<ServerCtx>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let p1 = [
        params["x1"].parse::<f64>().unwrap(),
        params["y1"].parse::<f64>().unwrap(),
    ];
    let p2 = [
        params["x2"].parse::<f64>().unwrap(),
        params["y2"].parse::<f64>().unwrap(),
    ];
    let aabb = AABB::from_corners(p1, p2);
    let type_filter = DamageType::from_bits_truncate(params["filter"].parse().unwrap());
    serde_json::to_string(
        &ctx.tree
            .locate_in_envelope(&aabb)
            .filter(|el| (el.damage_type & type_filter).into())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

pub async fn get_additional_info_for_point(
    State(ctx): State<Arc<ServerCtx>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    serde_json::to_string(&ctx.db.get_additional_info(id).await).unwrap()
}
