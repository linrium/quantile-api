use crate::pool::pool_router;
use warp::Filter;
#[macro_use]
extern crate dotenv_codegen;
extern crate pretty_env_logger;

mod common;
mod compute;
mod db;
mod pool;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let port = dotenv!("PORT").parse().expect("can not parse port");
    let db = db::Db::new();

    let pool_routes = pool_router::create_route(db);

    warp::serve(pool_routes.recover(common::response::handle_rejection))
        .run(([127, 0, 0, 1], port))
        .await;
}
