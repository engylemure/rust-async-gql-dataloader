pub mod user;

use async_graphql::Context;

use crate::models::User;
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    pub async fn empty(&self) -> &str {
        "empty"
    }

    pub async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        user::me(ctx)
    }
}
