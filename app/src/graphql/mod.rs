pub mod mutation;
pub mod objects;
pub mod query;
pub mod subscription;

use async_graphql::Schema as GqlSchema;

pub use mutation::Mutation;
pub use query::QueryRoot;
pub use subscription::Subscription;

pub type Schema = GqlSchema<QueryRoot, Mutation, Subscription>;
