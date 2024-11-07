use crate::data::models::{Permissions, ProjectModel, Ressource};
use crate::data::project_data::{create_project, delete_project, get_project, get_project_permission, get_projects, list_projects, update_project};
use crate::error::{not_found, unauthorized, Result};
use crate::messages::messages::SuccessMessage;
use crate::messages::project_messages::UpdateProjectMessage;
use crate::util::{Page, Pagination, Requirement};
use actix_web::{web, HttpRequest};
use sqlx::{PgConnection, Pool, Postgres};
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
            .route("", web::post().to(create_project_route))
            .route("/list", web::get().to(get_projects_route))
            .route("/{project_id}", web::get().to(get_project_route))
            .route("/{project_id}", web::delete().to(delete_project_route))
            .route("/{project_id}", web::put().to(update_project_route))
        );
}

async fn create_project_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<CreateProjectMessage>,
    req: HttpRequest,
) -> Result<web::Json<ProjectMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let mut new_project = msg.0.to_model(user.id);
    create_project(&mut *pool.acquire().await?, &mut new_project).await?;
    Ok(web::Json(new_project.into()))
}

async fn get_project_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<ProjectMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let project_id = path.into_inner();
    let mut conn = pool.acquire().await?;
    let proj: ProjectModel = get_project(&mut conn, project_id).await?.ok_or_else(|| not_found("Project Not Found"))?;
    let req = Permissions::Reader;
    let perms = help_get_project_permissions(&user, &proj, &mut conn).await?;
    if perms >= req {
        Ok(web::Json(proj.into()))
    } else {
        Err(unauthorized("Not allowed"))
    }
}


async fn get_projects_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    pag: Pagination,
) -> Result<web::Json<Page<ProjectMessage>>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let (est,projects) = if user.is_admin {
        list_projects(&mut *pool.acquire().await?, pag).await?
    } else {
        get_projects(&mut *pool.acquire().await?, user.id, pag).await?
    };
    Ok(web::Json(Page::new(projects.into_iter().map(Into::into).collect(), pag, est)))
}

async fn delete_project_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<SuccessMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let project_id = path.into_inner();
    let mut conn = pool.acquire().await?;
    let project= get_project(&mut *conn, project_id).await?.ok_or_else(|| not_found("Project not found"))?;
    let req = Permissions::Owner;
    let perms = help_get_project_permissions(&user, &project, &mut *conn).await?;
    if perms >= req {
        delete_project(&mut *conn, project_id).await?;
        Ok(web::Json(SuccessMessage::new(true)))
    } else {
        Err(unauthorized("Insufficient permissions on Project"))
    }
}


async fn update_project_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<UpdateProjectMessage>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<ProjectMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let project_id = path.into_inner();
    let mut conn = pool.acquire().await?;
    let update = msg.0;
    let mut project = get_project(&mut *conn, project_id).await?.ok_or_else(|| not_found("Project not found"))?;
    let req = update.requirements();
    let perms = help_get_project_permissions(&user, &project, &mut *conn).await?;
    if perms >= req {
        update.update_project(&mut project);
        update_project(&mut conn, &mut project).await?;
        Ok(web::Json(project.into()))
    } else {
        Err(unauthorized("Not Authorized to update project"))
    }
}



async fn help_get_project_permissions(user: &AuthenticatedUser, proj: &ProjectModel, conn: &mut PgConnection) -> Result<Permissions> {
    if user.is_admin {
        Ok(Permissions::Admin)
    } else if let Some(perms) = proj.get_permissions(user.id) {
        Ok(perms)
    } else {
        get_project_permission(conn, user.id, proj.id).await
    }
}