use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GQLResponse};
use async_graphql_actix_web::{GQLRequest, WSSubscription};
use sqlx::SqlitePool;

use crate::graphql::{context::Context, Schema};
use crate::models::SlimUser;

pub async fn gql(
    schema: web::Data<Schema>,
    pool: web::Data<SqlitePool>,
    gql_request: GQLRequest,
    user: SlimUser,
) -> web::Json<GQLResponse> {
    dbg!(&pool);
    let ctx = Context::new(pool.into_inner().as_ref().clone(), user).await;
    let req = gql_request.into_inner().data(ctx);
    web::Json(GQLResponse(req.execute(&schema).await))
}

pub async fn gql_subscriptions(
    schema: web::Data<Schema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}

pub async fn gql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source("/", Some("/"))))
}
