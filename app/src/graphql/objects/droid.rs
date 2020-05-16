use super::character::Character;
use crate::models::{Droid, Episode};
use async_graphql::Context;

#[async_graphql::Object]
impl Droid {
    pub async fn id(&self) -> &String {
        &self.id
    }

    pub async fn name(&self) -> &String {
        &self.name
    }

    pub async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        Vec::new()
    }

    pub async fn appears_in(&self, ctx: &Context<'_>) -> Vec<Episode> {
        Vec::new()
    }

    pub async fn primary_function(&self, ctx: &Context<'_>) -> &Option<String> {
        &self.primary_function
    }
}
