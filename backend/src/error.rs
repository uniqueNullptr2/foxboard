use std::num::ParseIntError;

use actix_web::{error::QueryPayloadError, http::header::ToStrError};
use derive_more::derive::{Display, Error};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Display, Error)]
pub enum AppError {
    IOError { msg: String },
    DBError { msg: String },
    CryptError { msg: String },
    RequestError { msg: String },
    AuthError { msg: String },
    NotFound { msg: String },
    Forbidden { msg: String },
    Error { msg: String },
    Unauthorized { msg: String },
}

impl actix_web::error::ResponseError for AppError {}

impl From<sqlx::error::Error> for AppError {
    fn from(item: sqlx::error::Error) -> Self {
        AppError::DBError {
            msg: item.to_string(),
        }
    }
}

impl From<QueryPayloadError> for AppError {
    fn from(item: QueryPayloadError) -> Self {
        AppError::RequestError {
            msg: item.to_string(),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(item: ParseIntError) -> Self {
        AppError::RequestError {
            msg: item.to_string(),
        }
    }
}

impl From<argon2::Error> for AppError {
    fn from(item: argon2::Error) -> Self {
        AppError::CryptError {
            msg: item.to_string(),
        }
    }
}

impl From<ToStrError> for AppError {
    fn from(item: ToStrError) -> Self {
        AppError::CryptError {
            msg: item.to_string(),
        }
    }
}

impl From<uuid::Error> for AppError {
    fn from(item: uuid::Error) -> Self {
        AppError::RequestError {
            msg: item.to_string(),
        }
    }
}

pub fn db_error(msg: &str) -> AppError {
    AppError::DBError {
        msg: msg.to_owned(),
    }
}

pub fn bad_request(msg: &str) -> AppError {
    AppError::RequestError {
        msg: msg.to_owned(),
    }
}

pub fn auth_error(msg: &str) -> AppError {
    AppError::AuthError {
        msg: msg.to_owned(),
    }
}

pub fn not_found(msg: &str) -> AppError {
    AppError::NotFound {
        msg: msg.to_owned(),
    }
}

pub fn forbidden(msg: &str) -> AppError {
    AppError::Forbidden {
        msg: msg.to_owned(),
    }
}

pub fn unauthorized(msg: &str) -> AppError {
    AppError::Unauthorized {
        msg: msg.to_owned(),
    }
}
