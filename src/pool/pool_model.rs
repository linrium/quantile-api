use serde::{Serialize, Deserialize};
use warp::reject;
use warp::http::StatusCode;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryPoolResult {
    pub quantile: f64,
    pub count: usize
}

#[derive(Debug)]
pub struct QueryPoolError {
    pub code: StatusCode,
    pub message: String
}

impl reject::Reject for QueryPoolError {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertPoolResult {
    pub status: String
}

#[derive(Debug)]
pub struct UpsertPoolError {
    pub code: StatusCode,
    pub message: String,
}

impl reject::Reject for UpsertPoolError {}
