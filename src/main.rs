use warp::Filter;
use std::error::Error;
use std::convert::Infallible;
use clap::Parser;

mod web_handler;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 1024)]
    port: u16,
}


#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>>{
    color_eyre::install()?;
    let args = Args::parse();
    start_server(args).await?;
    Ok(())
}

async fn start_server(args: Args) -> std::result::Result<(), Box<dyn Error>>{
    let root_path = warp::path::end()
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
