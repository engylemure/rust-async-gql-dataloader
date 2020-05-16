use crate::models::{Droid, Episode, Human};

#[async_graphql::Interface(
    field(name = "id", type = "&String", context),
    field(name = "name", type = "&String", context),
    field(name = "friends", type = "Vec<Character>", context),
    field(name = "appears_in", type = "Vec<Episode>", context)
)]
#[derive(Clone)]
pub enum Character {
    Human(Human),
    Droid(Droid),
}
