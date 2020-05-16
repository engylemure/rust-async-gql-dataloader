pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    pub async fn empty(&self) -> &str {
        "empty"
    }
}
