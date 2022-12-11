use std::env;
use log::info;
use clap::Parser;
use env_logger::Env;
use warp::{Filter, Rejection};
use crate::jwt_handler::{with_jwt, with_jwt_delete};
use crate::api_handler::add_json_body;

mod web_handler;
mod jwt_handler;
mod api_handler;
mod errors;
mod utilities;

type Result<T> = std::result::Result<T, errors::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 1024)]
    port: u16,
}


#[tokio::main]
async fn main() {
    setup_logging();
    let args = Args::parse();
    start_server(args).await;
}


fn setup_logging() {
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
}

async fn start_server(args: Args) {
    let index = warp::path::end()
        .and(warp::get())
        .and_then(web_handler::initial_load);

    let list_posts = warp::path("posts")
        .and(warp::get())
        .and_then(web_handler::get_post_list);

    let single_post = warp::path!("posts" / u16)
        .and(warp::get())
        .and_then(web_handler::get_post);


    let add = warp::path("add")
        .and(warp::post())
        .and(with_jwt())
        .and(add_json_body())
        .and_then(api_handler::add_post);

    let delete = warp::path!("delete" / u16)
        .and(warp::delete())
        .and(with_jwt_delete())
        .and_then(api_handler::delete_post);


    let routes = index
        .or(add)
        .or(single_post)
        .or(list_posts)
        .or(delete)
        .recover(errors::handle_rejection)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], args.port)).await;
}
