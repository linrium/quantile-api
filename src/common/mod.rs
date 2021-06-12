pub mod response;

use crate::db;
use warp::Filter;

pub fn with_db(
    db: db::Db,
) -> impl Filter<Extract = (db::Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
