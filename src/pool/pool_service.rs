use crate::pool::{pool_dto, pool_model};
use crate::{compute, db};
use warp::http::StatusCode;

pub async fn append_pool(
    mut db: db::Db,
    data: pool_dto::InsertDataDto,
) -> Result<pool_model::UpsertPoolResult, pool_model::UpsertPoolError> {
    let status = db
        .update_by_id(data.pool_id, data.pool_values)
        .await
        .map_err(|e| pool_model::UpsertPoolError {
            code: StatusCode::BAD_REQUEST,
            message: e.to_string(),
        })?;

    Ok(pool_model::UpsertPoolResult {
        status: status.to_string(),
    })
}

pub async fn query_pool(
    db: db::Db,
    data: pool_dto::QueryDataDto,
) -> Result<pool_model::QueryPoolResult, pool_model::QueryPoolError> {
    let pool_values = db.find_by_id(data.pool_id).await;

    if let Some(v) = pool_values {
        let quantile =
            compute::percentile(&v, data.percentile).map_err(|e| pool_model::QueryPoolError {
                code: StatusCode::BAD_REQUEST,
                message: e.to_string(),
            })?;

        return Ok(pool_model::QueryPoolResult {
            quantile,
            count: v.len(),
        });
    }

    Err(pool_model::QueryPoolError {
        code: StatusCode::NOT_FOUND,
        message: anyhow::anyhow!(db::DbError::PoolNotFound).to_string(),
    })
}

#[cfg(test)]
mod tests {
    use crate::db;
    use crate::db::UpdateStatus;
    use crate::pool::{pool_dto, pool_model, pool_service};
    use warp::hyper::http::StatusCode;

    #[tokio::test]
    async fn test_query_pool_success() -> Result<(), pool_model::QueryPoolError> {
        let db = db::Db::new();

        // mock insert
        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![1, 4, 2],
        };
        let _ = pool_service::append_pool(db.clone(), data).await.unwrap();

        // test query
        let data = pool_dto::QueryDataDto {
            pool_id: 1,
            percentile: 90_f64,
        };
        let result = pool_service::query_pool(db.clone(), data).await?;
        let expected = pool_model::QueryPoolResult {
            count: 3,
            quantile: 3.6,
        };

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_query_pool_not_found() -> Result<(), pool_model::QueryPoolError> {
        let db = db::Db::new();

        // test query
        let data = pool_dto::QueryDataDto {
            pool_id: 1,
            percentile: 90_f64,
        };
        let result = pool_service::query_pool(db.clone(), data)
            .await
            .err()
            .unwrap();
        let expected = pool_model::QueryPoolError {
            code: StatusCode::NOT_FOUND,
            message: db::DbError::PoolNotFound.to_string(),
        };

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_append_pool_insert_success() -> Result<(), pool_model::UpsertPoolError> {
        let db = db::Db::new();
        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![1, 4, 2],
        };
        let result = pool_service::append_pool(db.clone(), data).await?;
        let expected = pool_model::UpsertPoolResult {
            status: UpdateStatus::Inserted.to_string(),
        };

        assert_eq!(result, expected);

        // test sorted
        let result = db.find_by_id(1).await;
        let expected = vec![1, 2, 4];

        assert_eq!(result, Some(expected));
        Ok(())
    }

    #[tokio::test]
    async fn test_append_pool_update_success() -> Result<(), pool_model::UpsertPoolError> {
        let db = db::Db::new();
        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![1, 4, 5],
        };
        let _ = pool_service::append_pool(db.clone(), data).await?;

        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![2, 7],
        };
        let result = pool_service::append_pool(db.clone(), data).await?;

        let expected = pool_model::UpsertPoolResult {
            status: UpdateStatus::Appended.to_string(),
        };

        assert_eq!(result, expected);

        // test sorted
        let result = db.find_by_id(1).await;
        let expected = vec![1, 2, 4, 5, 7];

        assert_eq!(result, Some(expected));
        Ok(())
    }
}
