#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate sqlx;

mod db;
mod errors;
mod graphql;
mod models;
mod utils;
mod web_utils;

use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpServer};
use listenfd::ListenFd;

use crate::db::{mysql_pool, sqlite_pool};
use crate::graphql::{Mutation, QueryRoot, Schema, Subscription};
use crate::utils::env::ENV;
use crate::web_utils::handlers::{gql, gql_playground, gql_subscriptions};

fn create_schema() -> Schema {
    Schema::build(QueryRoot, Mutation, Subscription).finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = create_schema();
    let pool = sqlite_pool().await;
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/").guard(guard::Post()).to(gql))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(gql_subscriptions),
            )
            .service(web::resource("/").guard(guard::Get()).to(gql_playground))
    });

    server = if let Some(tcp_listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(tcp_listener).unwrap()
    } else {
        server.bind(format!("0.0.0.0:{}", ENV.server_port)).unwrap()
    };
    println!("Started http server: 0.0.0.0:{}", ENV.server_port);
    server.run().await
}
