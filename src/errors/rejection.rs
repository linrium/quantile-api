use crate::pool::pool_model;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let status_code;
    let msg;

    if err.is_not_found() {
        status_code = StatusCode::NOT_FOUND;
        msg = "NOT_FOUND".to_string();
    } else if let Some(pool_model::UpsertPoolError { message, code }) = err.find() {
        status_code = code.clone();
        msg = message.to_string();
    } else if let Some(pool_model::QueryPoolError { message, code }) = err.find() {
        status_code = code.clone();
        msg = message.to_string();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        status_code = StatusCode::BAD_REQUEST;

        msg = e.to_string();
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        status_code = StatusCode::METHOD_NOT_ALLOWED;
        msg = "METHOD_NOT_ALLOWED".to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        status_code = StatusCode::INTERNAL_SERVER_ERROR;
        msg = "UNHANDLED_REJECTION".to_string();
    }

    println!("{}", msg);
    let json = warp::reply::json(&ErrorMessage {
        code: status_code.as_u16(),
        message: msg.into(),
    });

    Ok(warp::reply::with_status(json, status_code))
}
