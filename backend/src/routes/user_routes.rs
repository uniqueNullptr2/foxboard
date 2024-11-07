use actix_web::{web, HttpRequest};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use rand::rngs::OsRng;
use rand::RngCore;

use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::models::{AuthenticatedUser, Permissions, UserModel, UserSessionModel};
use crate::data::user_data::{
    check_username_available, create_user, create_user_session, delete_user, get_user_by_id,
    get_user_by_name, get_users, update_user,
};
use crate::error::{bad_request, not_found, unauthorized, Result};
use crate::handler::user_handler::handle_token_update;
use crate::messages::general_messages::SuccessMessage;
use crate::messages::user_messages::{
    CreateUserMessage, LoginMessage, LoginTokenMessage, UpdateUserMessage, UserMessage,
};
use crate::util::{Page, Pagination};
pub fn register_user_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::scope("/user").route("/new", web::post().to(create_user)));
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user_route))
            .route("", web::put().to(update_user_route))
            .route("/list", web::get().to(get_users_route))
            .route(
                "/available/{username}",
                web::get().to(username_available_route),
            )
            .route("/{id}", web::delete().to(delete_user_route))
            .route("", web::get().to(get_user_route)),
    );
    cfg.route("/login", web::post().to(login_route));
}

// #[post("user/new/{invite_code}")]
async fn create_user_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<CreateUserMessage>,
    req: HttpRequest,
) -> Result<web::Json<UserMessage>> {
    log::info!("test");
    handle_token_update(&pool, &user, &user.token, &req).await?;
    user.is_admin()?;
    let mut new_user = UserModel::from(msg.0);
    create_user(&mut *pool.acquire().await?, &mut new_user).await?;
    Ok(web::Json(new_user.into()))
}

async fn login_route(
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<LoginMessage>,
    req: HttpRequest,
) -> Result<web::Json<LoginTokenMessage>> {
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
        Ok(web::Json(LoginTokenMessage::new(&token)))
    } else {
        Err(bad_request("username or password wrong"))
    }
}

async fn update_user_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<UpdateUserMessage>,
    req: HttpRequest,
) -> Result<web::Json<UserMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    msg.validate(&user)?;
    let req = msg.get_requirements(user.id);
    let perms = get_user_permissions(&user, msg.id.unwrap_or(user.id));
    if perms <= req {
        Err(unauthorized("You cannot do this"))
    } else {
        let mut conn = pool.acquire().await?;
        let mut mod_user = get_user_by_id(&mut conn, msg.id.unwrap_or(user.id))
            .await?
            .ok_or_else(|| not_found("User not found"))?;
        msg.0.update_user(&mut mod_user)?;
        update_user(&mut conn, &mod_user).await?;
        Ok(web::Json(mod_user.into()))
    }
}

async fn get_users_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    pag: Pagination,
    req: HttpRequest,
) -> Result<web::Json<Page<UserMessage>>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    user.is_admin()?;
    let (estimate, users) = get_users(&mut *pool.acquire().await?, pag).await?;
    Ok(web::Json(Page::new(
        users.into_iter().map(Into::into).collect(),
        pag,
        estimate,
    )))
}

async fn username_available_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<web::Json<SuccessMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let username = path.into_inner();
    Ok(web::Json(SuccessMessage::new(
        check_username_available(&mut *pool.acquire().await?, &username).await?,
    )))
}

async fn delete_user_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<SuccessMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let id = path.into_inner();
    if user.is_admin || user.id == id {
        delete_user(&mut *pool.acquire().await?, id).await?;
        Ok(web::Json(SuccessMessage::new(true)))
    } else {
        Err(unauthorized("I cannot let you do that"))
    }
}

async fn get_user_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    query: web::Query<GetUserQuery>,
) -> Result<web::Json<UserMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let id = match query.user_id {
        Some(id) if user.is_admin().is_ok() => id,
        _ => user.id,
    };

    Ok(web::Json(
        get_user_by_id(&mut *pool.acquire().await?, id)
            .await?
            .map(Into::into)
            .ok_or_else(|| not_found("User not found"))?,
    ))
}

#[derive(Deserialize)]
struct GetUserQuery {
    pub user_id: Option<Uuid>,
}

fn get_user_permissions(user: &AuthenticatedUser, user_id: Uuid) -> Permissions {
    if user.is_admin {
        Permissions::Admin
    } else if user.id == user_id {
        Permissions::Owner
    } else {
        Permissions::None
    }
}
