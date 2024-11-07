use super::user_data::get_user_from_session;
use crate::{
    error::{auth_error, not_found, unauthorized, AppError, Result},
    messages::user_messages::CreateUserMessage,
};
use actix_web::{web, FromRequest};
use argon2::Config;
use rand::{rngs::OsRng, RngCore};
use sqlx::{postgres::PgRow, types::Uuid, FromRow, Pool, Postgres, Row};
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
            Err(unauthorized("Admin required"))
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

    pub fn perms(&self) -> Option<Permissions> {
        if self.is_admin {
            Some(Permissions::Admin)
        } else {
            None
        }
    }
}

impl Ressource for UserModel {
    fn get_permissions(&self, user_id: Uuid) -> Option<Permissions> {
        if self.id == user_id {
            Some(Permissions::Owner)
        } else {
            None
        }
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
            is_admin: value.is_admin.unwrap_or(false),
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

impl Ressource for ProjectModel {
    fn get_permissions(&self, user_id: Uuid) -> Option<Permissions> {
        if user_id == self.owner_id {
            Some(Permissions::Owner)
        } else if self.public {
            Some(Permissions::Reader)
        } else {
            None
        }
    }
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
    pub column_id: Option<Uuid>,
    pub assignee_id: Option<Uuid>,
    pub creator_id: Option<Uuid>,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub estimation: Option<i32>,
    pub state_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub task_type: i32,
}

#[derive(Debug)]
pub enum TaskType {
    Standard,
    Repeatable,
}

impl Default for TaskType {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(sqlx::FromRow, Default, Debug)]
pub struct LabelModel {
    pub name: String,
    pub id: Uuid,
    pub project_id: Uuid,
}

#[derive(sqlx::FromRow, Default, Debug)]
pub struct StateModel {
    pub name: String,
    pub id: Uuid,
    pub project_id: Uuid,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Permissions {
    Admin,
    Owner,
    Editor,
    Reader,
    None,
}

impl Permissions {
    pub fn from_i32(i: i32) -> Self {
        match i {
            0 => Self::Admin,
            1 => Self::Owner,
            2 => Self::Editor,
            3 => Self::Reader,
            _ => Self::None,
        }
    }
}
impl FromRow<'_, PgRow> for Permissions {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self::from_i32(row.try_get(0)?))
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Self::None
    }
}
pub trait Ressource {
    fn get_permissions(&self, user_id: Uuid) -> Option<Permissions>;
}
