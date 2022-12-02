use std::env;
use log::info;
use warp::Filter;
use std::error::Error;
use clap::Parser;
use env_logger::Env;

mod web_handler;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 1024)]
    port: u16,
}


#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    setup_logging()?;
    let args = Args::parse();
    start_server(args).await?;
    Ok(())
}


fn setup_logging() -> std::result::Result<(), Box<dyn Error>> {
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
    Ok(())
}

async fn start_server(args: Args) -> std::result::Result<(), Box<dyn Error>> {
    let root_path = warp::path("init")
        .and(warp::get())
        .and_then(web_handler::initial_load);

    // let status_route = warp::path("posts").and_then(status::status_handler);
    // let catchall_route = warp::any()
    //     .and(warp::path::full())
    //     .and(warp::method())
    //     .and_then(handler::unhandled);

    let routes = root_path
        // .or(status_route)
        // .or(catchall_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], args.port)).await;
    Ok(())
}
