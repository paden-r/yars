use chrono::Utc;
use log::info;
use warp::Reply;
use warp::http::StatusCode;
use mysql::*;
use mysql::prelude::Queryable;
use std::error::Error;

use warp::{Filter, Rejection};

type HttpResult<T> = std::result::Result<T, Rejection>;

#[derive(Debug, PartialEq, Eq)]
struct Post {
    post_id: u32,
    create_date: String,
    title: String
}

#[derive(Debug, PartialEq, Eq)]
struct PostBody {
    body_id: u32,
    post_body: String,
}


fn get_opts() -> OptsBuilder {
    let db_user = std::env::var("DB_USER").unwrap();
    let db_pass = std::env::var("DB_PASS").unwrap();
    let db_port_string = std::env::var("DB_PORT").unwrap();
    let db_port: u16 = db_port_string.parse().unwrap();

    let mut builder = OptsBuilder::new();
    builder = builder.user(Some(db_user))
        .pass(Some(db_pass))
        .db_name(Some("yars"))
        .tcp_port(db_port)
        .ip_or_hostname(Some("[::1]"));

    builder
}


pub async fn initial_load() -> HttpResult<impl Reply> {
    info!("Now: {}", Utc::now().naive_utc());
    let posts = get_posts().await;
    println!("{:?}", posts);
    Ok(StatusCode::OK)
}


async fn get_posts() -> Vec<Post> {
    let mut connection = Conn::new(get_opts()).unwrap();
    let selected_posts = connection
        .query_map(
            "CALL GetPosts();",
        |(post_id, create_date, title)|{
            Post{post_id, create_date, title}
        },
        ).unwrap();

    selected_posts
}