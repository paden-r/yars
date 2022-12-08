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

type HttpResult<T> = std::result::Result<T, Rejection>;


#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct AddPostBody {
    title: String,
    ranking: String,
    summary: String,
    bodytext: String,
}


pub fn add_json_body() -> impl Filter<Extract=(AddPostBody, ), Error=Rejection> + Clone {
    warp::body::json()
}

pub async fn add_post(request_id: String, post_body: AddPostBody) -> HttpResult<impl Reply> {
    info!("Now: {}", Utc::now().naive_utc());
    let mut connection = Conn::new(get_opts()).unwrap();
    let sql_statement = format!(
        "CALL CreatePost({}, {}, {}, {});", post_body.title, post_body.bodytext, post_body.summary, post_body.ranking
    );
    connection.query_drop(sql_statement).unwrap();
    Ok(StatusCode::OK)
}
