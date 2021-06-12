use crate::db;
use crate::pool::{pool_dto, pool_service, pool_model};

pub async fn append_pool(
    db: db::Db,
    caching: db::Caching,
    data: pool_dto::InsertDataDto,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    caching.remove(data.pool_id);
    // println!("clear cached");
    let result = pool_service::append_pool(db, data.clone()).await?;

    Ok(Box::new(warp::reply::json(&result)))
}

pub async fn query_pool(
    db: db::Db,
    caching: db::Caching,
    data: pool_dto::QueryDataDto,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let cache_value = caching.get(data.pool_id, data.percentile);
    let result = if let Some(v) = cache_value {
        // println!("hit cached");
        pool_model::QueryPoolResult {
            quantile: v.0,
            count: v.1,
        }
    } else {
        // println!("set cached");
        let result = pool_service::query_pool(db, data).await?;
        caching.set(data.pool_id, data.percentile, (result.quantile, result.count));

        result
    };

    Ok(Box::new(warp::reply::json(&result)))
}
