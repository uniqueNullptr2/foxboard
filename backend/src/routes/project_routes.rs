use crate::error::Result;
use crate::handler::project_handler::{handle_create_project, handle_delete_project, handle_get_project, handle_get_projects};
use crate::util::{Page, Pagination};
use actix_web::{web, HttpRequest};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
// use crate::handler::project_handler::handle_create_project;
use crate::handler::user_handler::handle_token_update;
use crate::{
    data::models::AuthenticatedUser,
    messages::project_messages::{CreateProjectMessage, ProjectMessage},
};

pub fn register_project_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::scope("/user").route("/new", web::post().to(create_user)));
    cfg.service(
        web::scope("/projects")
            .route("", web::post().to(create_project))
            .route("/list", web::get().to(get_projects))
            .route("/{project_id}", web::get().to(get_project))
            .route("/{project_id}", web::delete().to(delete_project))
        );
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

async fn get_project(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<ProjectMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let id = path.into_inner();
    let proj = handle_get_project(&user, id,&pool).await?;
    Ok(web::Json(proj))
}


async fn get_projects(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    pag: Pagination,
) -> Result<web::Json<Page<ProjectMessage>>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let proj = handle_get_projects(pag, &user, &pool).await?;
    Ok(web::Json(proj))
}

async fn delete_project(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<String> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let project_id = path.into_inner();
    handle_delete_project(project_id, &user, &pool).await?;
    Ok("".to_owned())
}