mod db;
mod handler;
mod error;

use db::mod_db;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::NoTls;
use std::convert::Infallible;
use warp::{
    http::{header, Method},
    Filter, Rejection,
};

type Result<T> = std::result::Result<T, Rejection>;
type DBConn = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;


#[tokio::main]
async fn main() {
    let pool = mod_db::create_pool().expect("cannot create database pool");

    mod_db::db_init(&pool).await.expect("cannot initialize database");

    // url path
    let device = warp::path!("device" / i32);
    let device_post = warp::path("device");
    let owner = warp::path("owner");

    // device routes
    let device_route =
    device
    .and(warp::get())
    .and(with_db(pool.clone()))
    .and_then(handler::list_device_handler)
    .or(device_post
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handler::create_device_handler))
    .or(device
        .and(warp::delete())
        .and(with_db(pool.clone()))
        .and_then(handler::delete_device_handler)
    );

    let owner_route =
    owner
    .and(warp::get())
    .and(warp::path::param())
    .and(with_db(pool.clone()))
    .and_then(handler::get_owner_handler)
    .or(owner
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handler::list_owner_handler)
    )
    .or(owner
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handler::create_owner_handler)
    );


    let routes = device_route
        .or(owner_route)
        .recover(error::handle_rejection)
        .with(warp::cors()
            .allow_credentials(true)
            .allow_methods(&[
                Method::OPTIONS,
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PUT,
            ])
            .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .expose_headers(vec![header::LINK])
            .max_age(300)
            .allow_any_origin(),
        );

    warp::serve(routes).run(([127,0,0,1], 7878)).await;

}


fn with_db(pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}