use chrono::Utc;
use log::info;
use warp::Reply;
use warp::http::StatusCode;
use mysql::*;
use mysql::prelude::Queryable;
use serde::{Serialize, Deserialize};
use crate::utilities::get_opts;
use warp::{Rejection, Filter};
use warp::reply::{json, with_status};
use base64;
use std::str;

type HttpResult<T> = std::result::Result<T, Rejection>;


#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct AddPostBody {
    title: String,
    post_body: String,
    ranking: String,
    summary: String,
}


pub fn add_json_body() -> impl Filter<Extract=(AddPostBody, ), Error=Rejection> + Clone {
    warp::body::json()
}

pub async fn add_post(request_id: String, post_body: AddPostBody) -> HttpResult<impl Reply> {
    info!("Now: {}", Utc::now().naive_utc());
    let mut connection = Conn::new(get_opts()).unwrap();
    let bodytext_bytes = base64::decode(&post_body.post_body).unwrap();
    let bodytext = str::from_utf8(&bodytext_bytes).unwrap().to_owned();
    let sql_statement = format!(
        "CALL CreatePost('{}', '{}', '{}', '{}');",
        post_body.title.replace("'", "''"),
        bodytext.replace("'", "''"),
        post_body.summary.replace("'", "''"),
        post_body.ranking.replace("'", "''")
    );
    connection.query_drop(sql_statement).unwrap();
    Ok(StatusCode::OK)
}
