use async_graphql::{Context as GqlContext, FieldError};
use crate::graphql::context::Context;
use crate::models::{User, ProvideUser};
use crate::{errors::SWError, graphql::objects::user::Token};

pub type AuthResult = Result<Token, FieldError>;

pub async fn register(context: &GqlContext<'_>, email: String, password: String) -> AuthResult {
    let ctx = context.data::<Context>();
    let mut tx = ctx.pool.begin().await?;
    let user = User::new(email, password);
    let result = tx.create_user(user).await?;
    Ok(Token::from_user(result)?)
}