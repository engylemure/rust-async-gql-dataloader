use super::Episode;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Human {
    pub id: String,
    pub name: String,
    pub home_planet: Option<String>,
    pub friends: Vec<String>,
    pub appears_in: Vec<String>,
}
