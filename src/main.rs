use pothole_detection_backend::run;
use std::env;

#[tokio::main]
async fn main() {
    let db_url = env::var("REDIS_URL").expect("`REDIS_URL` is not set.");
    run(&db_url).await;
}
