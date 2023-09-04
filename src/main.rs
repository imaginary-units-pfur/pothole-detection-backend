use pothole_detection_backend::run;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db_url = env::var("REDIS_URL").expect("`REDIS_URL` is not set.");
    run(&db_url).await;
}
