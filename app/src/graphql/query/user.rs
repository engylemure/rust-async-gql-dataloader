use async_graphql::Context as GqlContext;
use crate::graphql::context::Context;
use crate::models::User;

pub fn me(ctx: &GqlContext) -> Option<User> {
    let ctx = ctx.data::<Context>();
    ctx.user.clone()
}

// pub fn me(context: &Context) -> Result<User, ServiceError> {
//     match context.user.clone() {
//         Some(user) => Ok(user),
//         None => Err(ServiceError::Unauthorized),
//     }
// }