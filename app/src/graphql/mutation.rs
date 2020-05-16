pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    pub async fn empty(&self) -> &str {
        "empty"
    }
}
