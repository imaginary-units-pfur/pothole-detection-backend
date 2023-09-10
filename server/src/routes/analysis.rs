use std::env;
use std::path::Path;
use std::str::FromStr;
use std::{io::Read, sync::Arc};

use axum::{extract::State, http::StatusCode, Json};
use common_data::DamageType;
use flate2::read::GzDecoder;

use crate::detection::predict;
use crate::ServerCtx;

#[derive(serde::Deserialize)]
pub struct RosDataPackage {
    pub longitude: f64,
    pub latitude: f64,
    pub image_shape: (u32, u32),
    pub image_encoding: String,
    pub image_bigendian: bool,
    pub image_step: usize,
    pub image_data_gzip_base64: String,
}

#[axum::debug_handler]
pub async fn analyze_ros_message(
    State(ctx): State<Arc<ServerCtx>>,
    Json(data): Json<RosDataPackage>,
) {
    use base64::Engine;
    let pixels_gzip = base64::engine::general_purpose::STANDARD
        .decode(data.image_data_gzip_base64)
        .unwrap();
    let mut decoder = GzDecoder::new(&pixels_gzip[..]);
    let mut pixels = vec![];
    decoder.read_to_end(&mut pixels).unwrap();
    let output_path = format!(
        "{}/{}_{}.jpg",
        env::var("IMAGE_DIR").unwrap(),
        data.longitude,
        data.latitude
    );
    let res = predict(pixels.as_slice(), &output_path).unwrap();
    if let Some(max) = res.iter().max_by(|x, y| x.1.total_cmp(&y.1)) {
        let damage_type = res
            .iter()
            .fold(0u16, |acc, el| 0u16 | DamageType::from_str(&el.0).unwrap());
        let id = ctx.db.insert_new(
            damage_type,
            Path::new(&output_path),
            data.latitude,
            data.longitude,
            max.1,
            &max.0,
        );
        ctx.tree.insert(common_data::RoadDamage {
            id,
            damage_type,
            latitude: data.latitude,
            longitude: data.longitude,
        });
    };
}
