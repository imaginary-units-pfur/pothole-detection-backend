use common_data::RoadDamage;

pub struct AABB {
    pub p1: (f64, f64),
    pub p2: (f64, f64),
}

pub async fn get_points_in_rect(
    server_address: &str,
    rect: AABB,
) -> anyhow::Result<Vec<RoadDamage>> {
    Ok(reqwest::get(format!(
        "{server_address}/points?x1={}&y1={}&x2={}&y2={}",
        rect.p1.0, rect.p1.1, rect.p2.0, rect.p2.1
    ))
    .await?
    .json::<Vec<RoadDamage>>()
    .await?)
}
