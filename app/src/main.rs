#[macro_use]
extern crate lazy_static;

mod db;
mod errors;
mod graphql;
mod models;
mod utils;

use actix_cors::Cors;
use actix_web::{guard, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GQLResponse};
use async_graphql_actix_web::{GQLRequest, WSSubscription};
use listenfd::ListenFd;

use crate::graphql::{Mutation, QueryRoot, Schema, Subscription};
use crate::utils::env::ENV;

async fn gql(schema: web::Data<Schema>, gql_request: GQLRequest) -> web::Json<GQLResponse> {
    web::Json(GQLResponse(gql_request.into_inner().execute(&schema).await))
}

async fn gql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}

async fn index_ws(
    schema: web::Data<Schema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = Schema::build(QueryRoot, Mutation, Subscription).finish();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
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
                    .to(index_ws),
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
