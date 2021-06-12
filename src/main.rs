use crate::pool::pool_router;
use warp::Filter;

mod db;
mod common;
mod pool;
mod compute;

#[tokio::main]
async fn main() {
    let db = db::Db::new();

    let pool_routes = pool_router::create_route(db);

    println!("Server is running on port 3030");
    warp::serve(pool_routes.recover(common::response::handle_rejection))
        .run(([127, 0, 0, 1], 3030))
        .await;
}