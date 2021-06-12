use crate::pool::{pool_dto, pool_model};
use crate::{compute, db};
use warp::http::StatusCode;

pub async fn upsert_pool(mut db: db::Db, data: pool_dto::InsertDataDto) -> Result<pool_model::UpsertPoolResult, pool_model::UpsertPoolError> {
    let status = db.update_by_id(data.pool_id, data.pool_values, true).await
        .map_err(|e| pool_model::UpsertPoolError {
            code: StatusCode::BAD_REQUEST,
            message: e.to_string()
        })?;

    Ok(pool_model::UpsertPoolResult {
        status: status.to_string()
    })
}

pub async fn query_pool(db: db::Db, data: pool_dto::QueryDataDto) -> Result<pool_model::QueryPoolResult, pool_model::QueryPoolError> {
    let pool_values = db.find_by_id(data.pool_id).await;

    if let Some(v) = pool_values {
        let quantile = compute::percentile(&v, data.percentile)
            .map_err(|e| pool_model::QueryPoolError {
                code: StatusCode::BAD_REQUEST,
                message: e.to_string(),
            })?;

        return Ok(pool_model::QueryPoolResult {
            quantile,
            count: v.len()
        });
    }

    Err(pool_model::QueryPoolError {
        code: StatusCode::NOT_FOUND,
        message: anyhow::anyhow!(db::DbError::PoolNotFound).to_string(),
    })
}
