use chrono::Utc;
use log::info;
use warp::Reply;
use warp::http::StatusCode;
use mysql::*;
use mysql::prelude::Queryable;
use serde::{Serialize, Deserialize};

use warp::{Rejection, Filter};
use warp::reply::{json, with_status};

type HttpResult<T> = std::result::Result<T, Rejection>;


#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
struct Post {
    post_id: u16,
    create_date: String,
    title: String,
    summary: String,
    ranking: String
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Deserialize)]
struct PostBody {
    post_id: u16,
    create_date: String,
    title: String,
    ranking: String,
    summary: String,
    bodytext: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct InitialLoadReturn {
    initial_posts: Vec<Post>,
    headliner: PostBody,
}


#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct AddPostBody {
    title: String,
    ranking: String,
    summary: String,
    bodytext: String,
}


pub fn add_json_body() -> impl Filter<Extract = (AddPostBody,), Error = Rejection> + Clone {
    warp::body::json()
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
        .ip_or_hostname(Some("127.0.0.1"));

    builder
}


pub async fn initial_load() -> HttpResult<impl Reply> {
    info!("Now: {}", Utc::now().naive_utc());
    let posts = get_posts().await;
    let headliner_id = posts[0].post_id.clone();
    let headliner = get_single_post(headliner_id).await;
    let return_data = if posts.len() > 1 {
        InitialLoadReturn {
            initial_posts: vec![posts[0].clone(), posts[1].clone()],
            headliner,
        }
    } else {
        InitialLoadReturn {
            initial_posts: vec![posts[0].clone()],
            headliner,
        }
    };
    info!("{:?}", &return_data);
    Ok(with_status(json(&return_data), StatusCode::OK))
}

pub async fn add_post(request_id: String, post_body: AddPostBody) -> HttpResult<impl Reply> {
    info!("Now: {}", Utc::now().naive_utc());
    // let mut connection = Conn::new(get_opts()).unwrap();
    // let sql_statement = format!("CALL CreatePost({});", post_id);
    Ok(StatusCode::OK)
}


async fn get_posts() -> Vec<Post> {
    let mut connection = Conn::new(get_opts()).unwrap();
    let selected_posts = connection
        .query_map(
            "CALL GetPosts();",
            |(post_id, create_date, title, summary, ranking)| {
                Post { post_id, create_date, title, summary, ranking}
            },
        ).unwrap();

    selected_posts
}


async fn get_single_post(post_id: u16) -> PostBody {
    let mut connection = Conn::new(get_opts()).unwrap();
    let sql_statement = format!("CALL GetPostBody({});", post_id);
    let mut single_post = connection
        .query_map(
            sql_statement,
            |(post_id, create_date, title, summary, ranking, bodytext)| {
                PostBody { post_id, create_date, title, summary, ranking, bodytext }
            },
        ).unwrap();

    single_post.pop().unwrap()
}
