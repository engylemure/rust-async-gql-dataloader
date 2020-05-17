use chrono::*;
use crate::utils::argon::{make_hash, make_salt};
use cuid::cuid;
use async_trait::async_trait;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub hash: Vec<u8>,
    pub salt: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn new(email: String, password: String) -> Self {
        let salt = make_salt();
        let hash = make_hash(&password, &salt);
        let id = cuid().expect("Could not create Collision unique ID");
        let now = Utc::now().timestamp();
        Self {
            id,
            hash,
            salt,
            email,
            created_at: now,
            updated_at: now
        }
    }
}

#[async_trait]
pub trait ProvideUser {
    async fn create_user(&mut self, user: User) -> sqlx::Result<User>;
}

#[async_trait]
impl ProvideUser for sqlx::SqliteConnection {
    async fn create_user(&mut self, user: User) -> sqlx::Result<User> {
        let result = sqlx::query("insert into users (id, hash, salt, email, created_at, updated_at) values ($1, $2, $3, $4, $5, $6)")
        .bind(&user.id)
        .bind(&user.hash)
        .bind(&user.salt)
        .bind(&user.email)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(self)
        .await;
        result.map(move |_| user)
    }
}

#[async_trait]
impl ProvideUser for sqlx::Transaction<sqlx::SqliteConnection> {
    async fn create_user(&mut self, user: User) -> sqlx::Result<User> {
        let result = sqlx::query("insert into users (id, hash, salt, email, created_at, updated_at) values ($1, $2, $3, $4, $5, $6)")
        .bind(&user.id)
        .bind(&user.hash)
        .bind(&user.salt)
        .bind(&user.email)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(self)
        .await;
        result.map(move |_| user)
    }
}

#[async_trait]
impl ProvideUser for sqlx::Transaction<sqlx::pool::PoolConnection<sqlx::SqliteConnection>> {
    async fn create_user(&mut self, user: User) -> sqlx::Result<User> {
        let result = sqlx::query("insert into users (id, hash, salt, email, created_at, updated_at) values ($1, $2, $3, $4, $5, $6)")
        .bind(&user.id)
        .bind(&user.hash)
        .bind(&user.salt)
        .bind(&user.email)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(self)
        .await;
        result.map(move |_| user)
    }
}



pub struct SlimUser {
    pub id: Option<String>,
}

impl Default for SlimUser {
    fn default() -> Self {
        Self { id: None }
    }
}
