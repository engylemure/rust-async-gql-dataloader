use async_graphql::Context;

pub mod user;
pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    pub async fn empty(&self) -> &str {
        "empty"
    }

    pub async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> user::AuthResult {
        user::register(ctx, email, password).await
    }
}
