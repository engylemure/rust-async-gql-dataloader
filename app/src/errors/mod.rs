use std::error::Error;
use std::fmt::Display;
use async_graphql::FieldError;
use serde_json::json;
use sqlx::Error as dbError;
#[derive(Debug)]
pub enum SWError {
    InternalServerError,
    Unauthorized,
    NotFound
}

impl From<SWError> for FieldError {
    fn from(err: SWError) -> Self {
        let (title, extensions) = match err {
            SWError::InternalServerError => (
                "InternalServerError",
                json!("")
            ),
            SWError::Unauthorized => (
                "Unauthorized",
                json!("")
            ),
            SWError::NotFound => (
                "NotFound",
                json!("")
            )
        };
        FieldError(title.to_string(), Some(extensions))
    }
}

impl From<dbError> for SWError {
    fn from(err: dbError) -> Self {
        match err {
            dbError::RowNotFound => SWError::NotFound,
            _ => SWError::InternalServerError
        }
    }
}