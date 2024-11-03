use actix_web::{web, HttpRequest};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::models::AuthenticatedUser;
use crate::error::Result;
use crate::handler::user_handler::{
    handle_check_username_available, handle_create_user, handle_delete_user, handle_get_user,
    handle_get_users, handle_login, handle_token_update, handle_update_user,
};
use crate::messages::messages::SuccessMessage;
use crate::messages::user_messages::{
    CreateUserMessage, LoginMessage, LoginTokenMessage, UpdateUserMessage, UserMessage,
};
use crate::util::{Page, Pagination};
pub fn register_user_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::scope("/user").route("/new", web::post().to(create_user)));
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("", web::put().to(update_user))
            .route("/list", web::get().to(get_users))
            .route("/available/{username}", web::get().to(username_available))
            .route("/{id}", web::delete().to(delete_user))
            .route("", web::get().to(get_user)),
    );
    cfg.route("/login", web::post().to(login));
}

// #[post("user/new/{invite_code}")]
async fn create_user(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<CreateUserMessage>,
    req: HttpRequest,
) -> Result<web::Json<UserMessage>> {
    log::info!("test");
    handle_token_update(&pool, &user, &user.token, &req).await?;
    user.is_admin()?;
    let new_user = handle_create_user(msg.0, &pool).await?;
    Ok(web::Json(new_user))
}

async fn login(
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<LoginMessage>,
    req: HttpRequest,
) -> Result<web::Json<LoginTokenMessage>> {
    let token_res = handle_login(msg.0, &pool, &req).await?;
    Ok(web::Json(token_res))
}

async fn update_user(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<UpdateUserMessage>,
    req: HttpRequest,
) -> Result<web::Json<UserMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let mod_user = handle_update_user(&user, msg.0, &pool).await?;
    Ok(web::Json(mod_user))
}

async fn get_users(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    pag: Pagination,
    req: HttpRequest,
) -> Result<web::Json<Page<UserMessage>>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    user.is_admin()?;
    let users = handle_get_users(&pool, pag).await?;
    Ok(web::Json(users))
}

async fn username_available(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<web::Json<SuccessMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let username = path.into_inner();
    Ok(web::Json(
        handle_check_username_available(&pool, &username).await?,
    ))
}

async fn delete_user(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<String> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let id = path.into_inner();
    handle_delete_user(&pool, &user, id).await?;
    Ok("".to_owned())
}

async fn get_user(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    query: web::Query<GetUserQuery>,
) -> Result<web::Json<UserMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let user_msg = match query.user_id {
        Some(id) if user.is_admin().is_ok() => handle_get_user(&pool, id).await?,
        _ => handle_get_user(&pool, user.id).await?,
    };

    Ok(web::Json(user_msg))
}

#[derive(Deserialize)]
struct GetUserQuery {
    pub user_id: Option<Uuid>,
}
