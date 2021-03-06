use crate::pool::pool_router;
use warp::Filter;
#[macro_use]
extern crate dotenv_codegen;
extern crate pretty_env_logger;

mod errors;
mod compute;
mod db;
mod pool;
mod utils;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let port = dotenv!("PORT").parse().expect("can not parse port");
    let db = db::Db::new();
    let caching = db::Caching::new();

    let pool_routes = pool_router::create_route(db, caching);

    warp::serve(pool_routes.recover(errors::rejection::handle_rejection))
        .run(([0, 0, 0, 0], port))
        .await;
}
