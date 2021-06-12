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

#[cfg(test)]
mod tests {
    use crate::db;
    use crate::db::UpdateStatus;
    use crate::pool::{pool_dto, pool_model, pool_router};
    use warp::hyper::http::StatusCode;

    #[tokio::test]
    async fn test_upsert_pool_insert_success() {
        let db = db::Db::new();
        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![1, 4, 2],
        };
        let api = pool_router::create_route(db);

        let resp = warp::test::request()
            .method("POST")
            .path("/upsert")
            .json(&data)
            .reply(&api)
            .await;

        let body = resp.body();
        let body: pool_model::UpsertPoolResult = serde_json::from_slice(body).unwrap();
        let expected = pool_model::UpsertPoolResult {
            status: UpdateStatus::Inserted.to_string(),
        };

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body, expected);
    }

    #[tokio::test]
    async fn test_upsert_pool_update_success() {
        let db = db::Db::new();
        let api = pool_router::create_route(db);

        // mock insert
        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![1, 4, 2],
        };
        let resp = warp::test::request()
            .method("POST")
            .path("/upsert")
            .json(&data)
            .reply(&api)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);

        // test update
        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![3, 7],
        };
        let resp = warp::test::request()
            .method("POST")
            .path("/upsert")
            .json(&data)
            .reply(&api)
            .await;

        let body = resp.body();
        let body: pool_model::UpsertPoolResult = serde_json::from_slice(body).unwrap();
        let expected = pool_model::UpsertPoolResult {
            status: UpdateStatus::Updated.to_string(),
        };

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body, expected);
    }

    #[tokio::test]
    async fn test_query_pool_success() {
        let db = db::Db::new();
        let api = pool_router::create_route(db);

        // mock insert
        let data = pool_dto::InsertDataDto {
            pool_id: 1,
            pool_values: vec![1, 4, 2],
        };
        let resp = warp::test::request()
            .method("POST")
            .path("/upsert")
            .json(&data)
            .reply(&api)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);

        // test update
        let data = pool_dto::QueryDataDto {
            pool_id: 1,
            percentile: 50.0,
        };
        let resp = warp::test::request()
            .method("POST")
            .path("/query")
            .json(&data)
            .reply(&api)
            .await;

        let body = resp.body();
        let body: pool_model::QueryPoolResult = serde_json::from_slice(body).unwrap();
        let expected = pool_model::QueryPoolResult {
            quantile: 2.0,
            count: 3,
        };

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body, expected);
    }

    #[tokio::test]
    async fn test_query_pool_not_found() {
        let db = db::Db::new();
        let api = pool_router::create_route(db);

        // test update
        let data = pool_dto::QueryDataDto {
            pool_id: 1,
            percentile: 50.0,
        };
        let resp = warp::test::request()
            .method("POST")
            .path("/query")
            .json(&data)
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
