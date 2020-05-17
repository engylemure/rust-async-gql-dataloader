use async_graphql::Context;
use chrono::*;

use crate::models::user::User;
use crate::{web_utils::jwt::create_token, errors::SWError};

#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> &String {
        &self.id
    }
    pub async fn email(&self) -> &String {
        &self.email
    }
    pub async fn created_at(&self) -> DateTime<Utc> {
        let date = NaiveDateTime::from_timestamp(self.created_at, 0);
        DateTime::<Utc>::from_utc(date, Utc)
    }
    pub async fn updated_at(&self) -> DateTime<Utc> {
        let date = NaiveDateTime::from_timestamp(self.updated_at, 0);
        DateTime::<Utc>::from_utc(date, Utc)    
    }
}


/// decrypted token JWT and return into login
pub struct Token {
    pub bearer: Option<String>,
    pub user: User,
}

#[async_graphql::Object]
impl Token {
    pub async fn bearer(&self) -> &Option<String> {
        &self.bearer
    }
    pub async fn user(&self) -> &User {
        &self.user
    }
}

impl Token {
    pub fn from_user(user: User) -> Result<Token, SWError> {
        match create_token(&user.id) {
            Ok(token) => Ok(Token {
                bearer: Some(token),
                user
            }),
            Err(_) => Err(SWError::InternalServerError)
        }
    }
}