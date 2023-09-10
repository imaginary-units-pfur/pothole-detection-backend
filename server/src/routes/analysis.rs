use std::{io::Read, sync::Arc};

use axum::{extract::State, http::StatusCode, Json};
use flate2::read::GzDecoder;

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
) -> () {
    use base64::Engine;
    let pixels_gzip = base64::engine::general_purpose::STANDARD
        .decode(data.image_data_gzip_base64)
        .unwrap();
    let mut decoder = GzDecoder::new(&pixels_gzip[..]);
    let mut pixels = vec![];
    decoder.read_to_end(&mut pixels).unwrap();

    unimplemented!("pixel processing stuff...");
}
