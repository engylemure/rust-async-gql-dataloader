use super::Episode;

#[derive(Clone)]
pub struct Droid {
    pub id: String,
    pub name: String,
    pub friends: Vec<String>,
    pub appears_in: Vec<String>,
    pub primary_function: Option<String>,
}
