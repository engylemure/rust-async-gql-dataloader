#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Episode {
    pub id: String,
    pub number: u8,
    pub name: String,
    pub released_at: String,
}
