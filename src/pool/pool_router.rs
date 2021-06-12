use crate::common::with_db;
use crate::db;
use crate::pool::pool_handler;
use warp::Filter;

pub fn create_route(
    db: db::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    upsert_pool_route(db.clone()).or(query_pool_route(db))
}

fn upsert_pool_route(
    db: db::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("upsert")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(pool_handler::upsert_pool)
}

fn query_pool_route(
    db: db::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("query")
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(pool_handler::query_pool)
}
