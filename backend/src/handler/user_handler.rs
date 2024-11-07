use actix_web::HttpRequest;
use sqlx::{Pool, Postgres};

use crate::data::models::UserSessionModel;
use crate::data::user_data::update_user_session;
use crate::error::{bad_request, Result};
use crate::{
    config::Config,
    data::{
        models::UserModel,
        user_data::{create_user, get_user_by_name},
    },
    messages::user_messages::CreateUserMessage,
};

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
