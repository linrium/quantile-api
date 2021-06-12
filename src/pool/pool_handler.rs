use crate::db;
use crate::pool::{pool_dto, pool_service};

pub async fn upsert_pool(db: db::Db, data: pool_dto::InsertDataDto) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let result = pool_service::upsert_pool(db, data).await?;

    Ok(Box::new(warp::reply::json(&result)))
}

pub async fn query_pool(db: db::Db, data: pool_dto::QueryDataDto) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let result = pool_service::query_pool(db, data).await?;

    Ok(Box::new(warp::reply::json(&result)))
}
