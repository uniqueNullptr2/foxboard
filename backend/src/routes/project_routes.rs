use crate::error::Result;
use crate::handler::project_handler::handle_create_project;
use actix_web::{web, HttpRequest};
use sqlx::{Pool, Postgres};
// use crate::handler::project_handler::handle_create_project;
use crate::handler::user_handler::handle_token_update;
use crate::{
    data::models::AuthenticatedUser,
    messages::project_messages::{CreateProjectMessage, ProjectMessage},
};

pub fn register_project_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::scope("/user").route("/new", web::post().to(create_user)));
    cfg.service(web::scope("/projects").route("", web::post().to(create_project)));
}

async fn create_project(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<CreateProjectMessage>,
    req: HttpRequest,
) -> Result<web::Json<ProjectMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let proj = handle_create_project(msg.0, user.id, &pool).await?;
    Ok(web::Json(proj))
}
