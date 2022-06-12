use warp::Filter;
use log::info;
use std::env;
use env_logger::Env;


#[tokio::main]
async fn main() {
    let mut logger = env_logger::Builder::from_env(Env::default().default_filter_or("info"));
    logger.target(env_logger::Target::Stdout);
    logger.init();
    match env::var("RUST_LOG") {
        Ok(l) => {
            info!("Log Level: {}", l.to_uppercase())
        }
        Err(_) => {
            info!("Log Level: INFO")
        }
    }

    let index = warp::path::end()
        .map(|| "Hello");

    warp::serve(index)
        .run(([0, 0, 0, 0], 1024))
        .await;
}
