use async_graphql::Context;

use super::character::Character;
use crate::models::{Episode, Human};

#[async_graphql::Object]
impl Human {
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

    pub async fn home_planet(&self, ctx: &Context<'_>) -> &Option<String> {
        &self.home_planet
    }
}
