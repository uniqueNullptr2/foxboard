use std::{
    collections::HashMap,
    future::{ready, Ready},
};

use actix_web::{web::Query, FromRequest};
use serde::{Deserialize, Serialize};

use crate::error::{bad_request, AppError, Result};

#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page: i32,
    pub count: i32,
}

impl Pagination {
    pub fn validate(&self) -> Result<()> {
        if self.page <= 0 || self.count <= 0 || self.count > 200 {
            Err(bad_request("pagination invalid"))
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page<T> {
    pub page: i32,
    pub count: i32,
    pub total: i32,
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, pag: Pagination, total: i32) -> Self {
        Self {
            page: pag.page,
            count: pag.count,
            total,
            items,
        }
    }
}

impl FromRequest for Pagination {
    type Error = AppError;

    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // TODO this is bad and probably can and should be improved
        let r = || {
            let params = Query::<HashMap<String, String>>::from_query(req.query_string())?;
            let page = params
                .get("page")
                .map(|p| p.parse::<i32>())
                .unwrap_or(Ok(1))?;
            let count = params
                .get("count")
                .map(|p| p.parse::<i32>())
                .unwrap_or(Ok(50))?;
            let pag = Self { page, count };
            pag.validate()?;
            Ok(pag)
        };
        ready(r())
    }
}
