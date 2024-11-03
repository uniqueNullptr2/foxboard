use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::models::{AuthenticatedUser, UserModel};
use crate::error::{forbidden, Result};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserMessage {
    pub(crate) username: String,
    pub(crate) password: String,
    // pub(crate) email: String,
    pub(crate) is_admin: Option<bool>,
}

impl CreateUserMessage {
    pub fn new(username: String, password: String, is_admin: bool) -> Self {
        Self {
            username,
            password,
            is_admin: Some(is_admin),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserMessage {
    pub(crate) username: String,
    // pub(crate) email: String,
    pub id: Uuid,
    pub(crate) is_admin: bool,
}

impl UserMessage {}

impl From<UserModel> for UserMessage {
    fn from(value: UserModel) -> Self {
        Self {
            username: value.username,
            is_admin: value.is_admin,
            id: value.id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginMessage {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginTokenMessage {
    pub token: String,
}

impl LoginTokenMessage {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserMessage {
    pub is_admin: Option<bool>,
    pub username: Option<String>,
    pub id: Option<Uuid>,
    pub new_password: Option<String>,
    pub old_password: Option<String>,
}

impl UpdateUserMessage {
    pub fn validate(&self, user: &AuthenticatedUser) -> Result<()> {
        if !user.is_admin
            && (self.is_admin.unwrap_or(false) || self.id.filter(|id| *id != user.id).is_some())
        {
            Err(forbidden("Not allowed to change a user this way."))
        } else {
            Ok(())
        }
    }

    pub fn update_user(self, mod_user: &mut UserModel) -> Result<()> {
        self.username
            .into_iter()
            .for_each(|u| mod_user.username = u);
        self.is_admin
            .into_iter()
            .for_each(|u| mod_user.is_admin = u);
        match &self.new_password {
            Some(u) => {
                if mod_user.check_password(&self.old_password.unwrap_or("".to_owned()))? {
                    mod_user.change_password(&u);
                }
            }
            _ => (),
        }
        Ok(())
    }
}
