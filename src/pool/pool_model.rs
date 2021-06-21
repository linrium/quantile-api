use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::reject;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryPoolResult {
    pub quantile: f32,
    pub count: usize,
}

#[derive(Debug, PartialEq)]
pub struct QueryPoolError {
    pub code: StatusCode,
    pub message: String,
}

impl reject::Reject for QueryPoolError {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpsertPoolResult {
    pub status: String,
}

#[derive(Debug)]
pub struct UpsertPoolError {
    pub code: StatusCode,
    pub message: String,
}

impl reject::Reject for UpsertPoolError {}
