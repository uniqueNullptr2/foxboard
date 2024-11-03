use actix_web::HttpRequest;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use rand::rngs::OsRng;
use rand::RngCore;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::models::{AuthenticatedUser, UserSessionModel};
use crate::data::user_data::{
    check_username_available, create_user_session, delete_user, get_user_by_id, get_users,
    update_user, update_user_session,
};
use crate::error::{bad_request, not_found, Result};
use crate::messages::messages::SuccessMessage;
use crate::messages::user_messages::{LoginMessage, LoginTokenMessage, UpdateUserMessage};
use crate::util::{Page, Pagination};
use crate::{
    config::Config,
    data::{
        models::UserModel,
        user_data::{create_user, get_user_by_name},
    },
    messages::user_messages::{CreateUserMessage, UserMessage},
};

pub async fn handle_create_user(
    msg: CreateUserMessage,
    pool: &Pool<Postgres>,
) -> Result<UserMessage> {
    let mut new_user = UserModel::from(msg);
    create_user(&mut *pool.acquire().await?, &mut new_user).await?;
    Ok(new_user.into())
}

pub async fn handle_create_initial_admin(config: &Config, pool: &Pool<Postgres>) -> Result<()> {
    let mut conn = pool.acquire().await?;
    let user = get_user_by_name(&mut conn, &config.admin_user).await?;

    if user.is_none() {
        create_user(
            &mut conn,
            &mut CreateUserMessage::new(
                config.admin_user.clone(),
                config.admin_initial_password.clone(),
                true,
            )
            .into(),
        )
        .await?;
    }
    Ok(())
}

pub async fn handle_login(
    msg: LoginMessage,
    pool: &Pool<Postgres>,
    req: &HttpRequest,
) -> Result<LoginTokenMessage> {
    let mut conn = pool.acquire().await?;
    let user = get_user_by_name(&mut conn, &msg.username)
        .await?
        .unwrap_or_default();
    let user_agent = req
        .headers()
        .get("User-Agent")
        .ok_or(bad_request("Missing User Agent Header"))?
        .to_str()?;
    let ip_addr = req
        .peer_addr()
        .ok_or(bad_request("IP Address Missing"))?
        .to_string();
    let mut token_bin = [0u8; 64];
    OsRng.fill_bytes(&mut token_bin);
    let token = BASE64_STANDARD.encode(token_bin);
    if user.check_password(&msg.password)? {
        create_user_session(
            &mut conn,
            UserSessionModel::new(&token, user_agent, ip_addr, user.id),
        )
        .await?;
        Ok(LoginTokenMessage::new(&token))
    } else {
        Err(bad_request("username or password wrong"))
    }
}

pub async fn handle_token_update(
    pool: &Pool<Postgres>,
    user: &UserModel,
    token: &str,
    req: &HttpRequest,
) -> Result<()> {
    let user_agent = req
        .headers()
        .get("User-Agent")
        .ok_or(bad_request("Missing User Agent Header"))?
        .to_str()?;
    let ip_addr = req
        .peer_addr()
        .ok_or(bad_request("IP Address Missing"))?
        .to_string();
    update_user_session(
        &mut *pool.acquire().await?,
        UserSessionModel::new(token, user_agent, ip_addr, user.id),
    )
    .await?;

    Ok(())
}

pub async fn handle_update_user(
    user: &AuthenticatedUser,
    msg: UpdateUserMessage,
    pool: &Pool<Postgres>,
) -> Result<UserMessage> {
    msg.validate(&user)?;
    let mut conn = pool.acquire().await?;
    let mut mod_user = get_user_by_id(&mut conn, msg.id.unwrap_or(user.id))
        .await?
        .ok_or_else(|| not_found("User not found"))?;
    msg.update_user(&mut mod_user)?;
    update_user(&mut conn, &mod_user).await?;
    Ok(mod_user.into())
}

pub async fn handle_get_users(pool: &Pool<Postgres>, pag: Pagination) -> Result<Page<UserMessage>> {
    let (estimate, users) = get_users(&mut *pool.acquire().await?, pag).await?;
    Ok(Page::new(
        users.into_iter().map(|u| u.into()).collect(),
        pag,
        estimate,
    ))
}

pub async fn handle_check_username_available(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<SuccessMessage> {
    Ok(SuccessMessage::new(
        check_username_available(&mut *pool.acquire().await?, username).await?,
    ))
}

pub async fn handle_delete_user(pool: &Pool<Postgres>, id: Uuid) -> Result<()> {
    delete_user(&mut *pool.acquire().await?, id).await?;
    Ok(())
}

pub async fn handle_get_user(pool: &Pool<Postgres>, id: Uuid) -> Result<UserMessage> {
    Ok(get_user_by_id(&mut *pool.acquire().await?, id)
        .await?
        .map(Into::into)
        .ok_or_else(|| not_found("User not found"))?)
}
