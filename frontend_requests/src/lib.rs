use common_data::{RoadDamage, RoaddamageAdditionalInfo};

#[derive(Debug, PartialEq, Clone, Copy)]
/// Points are in (longitude, latitude) format
pub struct AABB {
    pub p1: (f64, f64),
    pub p2: (f64, f64),
}

pub async fn get_points_in_rect(
    server_address: &str,
    rect: AABB,
) -> anyhow::Result<Vec<RoadDamage>> {
    Ok(reqwest::get(format!(
        "{server_address}/points/by-coords/from/{}/{}/to/{}/{}",
        rect.p1.0, rect.p1.1, rect.p2.0, rect.p2.1
    ))
    .await?
    .json::<Vec<RoadDamage>>()
    .await?)
}

pub async fn get_info_by_id(
    server_address: &str,
    id: i64,
) -> anyhow::Result<RoaddamageAdditionalInfo> {
    Ok(reqwest::get(format!("{server_address}/points/by-id/{id}"))
        .await?
        .json::<RoaddamageAdditionalInfo>()
        .await?)
}
