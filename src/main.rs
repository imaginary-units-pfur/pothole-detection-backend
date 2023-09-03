use pothole_detection_backend::run;
use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let ip = Ipv4Addr::from_str(&env::var("SERVER_IP").expect("`SERVER_IP` is not set."))
        .expect("Could not parse `SERVER_IP`.");
    let port = u16::from_str(&env::var("SERVER_PORT").expect("`SERVER_PORT` is not set."))
        .expect("Could not parse `SERVER_PORT`.");

    run(ip, port).await
}
