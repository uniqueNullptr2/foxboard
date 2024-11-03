use super::user_data::get_user_from_session;
use crate::{
    error::{auth_error, forbidden, not_found, AppError, Result},
    messages::user_messages::CreateUserMessage,
};
use actix_web::{web, FromRequest};
use argon2::Config;
use rand::{rngs::OsRng, RngCore};
use sqlx::{types::Uuid, Pool, Postgres};
use std::{future::Future, ops::Deref, pin::Pin};

#[derive(sqlx::FromRow, Default, Debug)]
pub struct UserModel {
    pub(crate) username: String,
    pub(crate) password_hash: String,
    // pub(crate) email: String,
    pub(crate) id: Uuid,
    pub(crate) is_admin: bool,
}

impl UserModel {
    pub fn change_password(&mut self, new_pw: &str) {
        let mut salt = [0u8; 12];
        OsRng.fill_bytes(&mut salt);

        let config = Config::default();
        self.password_hash = argon2::hash_encoded(new_pw.as_bytes(), &salt, &config).unwrap();
    }

    pub fn is_admin(&self) -> Result<()> {
        if !self.is_admin {
            Err(forbidden("Forbidden"))
        } else {
            Ok(())
        }
    }

    pub fn check_password(&self, password: &str) -> Result<bool> {
        Ok(argon2::verify_encoded(
            &self.password_hash,
            password.as_bytes(),
        )?)
    }
}

impl From<CreateUserMessage> for UserModel {
    fn from(value: CreateUserMessage) -> Self {
        let mut salt = [0u8; 12];
        OsRng.fill_bytes(&mut salt);
        let config = Config::default();
        let hash = argon2::hash_encoded(value.password.as_bytes(), &salt, &config).unwrap();

        Self {
            username: value.username,
            password_hash: hash,
            id: Uuid::nil(),
            is_admin: value.is_admin,
        }
    }
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: UserModel,
    pub token: String,
}

impl Deref for AuthenticatedUser {
    type Target = UserModel;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;

    type Future = Pin<Box<dyn Future<Output = std::result::Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let pool_res = req
            .app_data::<web::Data<Pool<Postgres>>>()
            .ok_or(AppError::DBError {
                msg: "Pool Missing".to_owned(),
            })
            .cloned();
        log::info!("here be no problems yet");
        let header = req
            .headers()
            .get("Authorization")
            .ok_or(auth_error("Auth header missing"))
            .cloned();
        Box::pin(async move {
            let pool = pool_res?;
            let h = header?;
            let token = h.to_str().map_err(|e| auth_error(&e.to_string()))?;
            if !token.starts_with("Bearer ") {
                Err(auth_error("Doesnt start with 'Bearer '"))?;
            }
            log::info!("{}", &token[7..]);
            let user = get_user_from_session(&mut *pool.acquire().await?, &token[7..])
                .await?
                .ok_or(not_found("Could not find user by token"))?;
            Ok(Self {
                user,
                token: token.to_owned(),
            })
        })
    }
}

#[derive(sqlx::FromRow, Default, Debug)]
pub struct UserSessionModel {
    pub token: String,
    pub user_agent: String,
    pub ip_addr: String,
    pub user_id: Uuid,
    //todo time
}

impl UserSessionModel {
    pub fn new(token: &str, user_agent: &str, ip_addr: String, user_id: Uuid) -> Self {
        Self {
            token: token.to_owned(),
            user_agent: user_agent.to_owned(),
            ip_addr,
            user_id,
        }
    }
}

#[derive(sqlx::FromRow, Default, Debug)]
pub struct ProjectModel {
    pub name: String,
    pub public: bool,
    pub id: Uuid,
    pub owner_id: Uuid,
}

#[derive(sqlx::FromRow, Default, Debug)]
pub struct ProjectColumnModel {
    pub name: String,
    pub card_limit: i32,
    pub id: Uuid,
    pub project_id: Uuid,
    pub index: i32,
}

#[derive(sqlx::FromRow, Default, Debug)]
pub struct TaskModel {
    pub title: String,
    pub id: Uuid,
    pub project_id: Uuid,
    pub column_id: Uuid,
    pub assignee_id: Uuid,
    pub creator_id: Uuid,
    pub deadline: chrono::DateTime<chrono::Utc>,
    pub estimation: i32,
}

#[derive(sqlx::FromRow, Default, Debug)]

pub struct LabelModel {
    pub name: String,
    pub id: Uuid,
    pub project_id: Uuid,
}
