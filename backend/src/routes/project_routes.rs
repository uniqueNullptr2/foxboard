use crate::data::models::{Permissions, ProjectModel, Ressource};
use crate::data::project_data::{
    create_column, create_label, create_project, delete_column, delete_label, delete_project, get_column, get_columns, get_label, get_labels, get_project, get_project_permission, get_projects, list_projects, update_column, update_label, update_project
};
use crate::error::{not_found, unauthorized, Result};
use crate::messages::general_messages::SuccessMessage;
use crate::messages::project_messages::{CreateLabelMessage, CreateProjectColumnMessage, LabelMessage, ProjectColumnMessage, UpdateLabelMessage, UpdateProjectColumnMessage, UpdateProjectMessage};
use crate::util::{Page, Pagination};
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
            .route("/{project_id}/columns", web::post().to(create_column_route))
            .route("/{project_id}/labels", web::post().to(create_label_route))
            .route("/{project_id}/columns/{column_id}", web::put().to(update_column_route))
            .route("/{project_id}/labels/{label_id}", web::put().to(update_label_route))
            .route("/{project_id}/columns/{column_id}", web::delete().to(delete_column_route))
            .route("/{project_id}/labels/{label_id}", web::delete().to(delete_label_route)),
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
    let proj: ProjectModel = get_project(&mut conn, project_id)
        .await?
        .ok_or_else(|| not_found("Project Not Found"))?;
    let req = Permissions::Reader;
    let perms = help_get_project_permissions(&user, &proj, &mut conn).await?;
    if perms <= req {
        let labels = get_labels(&mut conn, project_id).await?.into_iter().map(Into::into).collect();
        let columns = get_columns(&mut conn, project_id).await?.into_iter().map(Into::into).collect();
        let mut msg: ProjectMessage = proj.into();
        msg.labels = Some(labels);
        msg.columns = Some(columns);
        Ok(web::Json(msg))
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
    let (est, projects) = if user.is_admin {
        list_projects(&mut *pool.acquire().await?, pag).await?
    } else {
        get_projects(&mut *pool.acquire().await?, user.id, pag).await?
    };
    Ok(web::Json(Page::new(
        projects.into_iter().map(Into::into).collect(),
        pag,
        est,
    )))
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
    if user.is_admin || get_project_permission(&mut conn, user.id, project_id).await? >= Permissions::Owner{
        delete_project(&mut conn, project_id).await?;
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
    let mut project = get_project(&mut conn, project_id)
        .await?
        .ok_or_else(|| not_found("Project not found"))?;
    let req = update.get_requirements();
    let perms = help_get_project_permissions(&user, &project, &mut conn).await?;
    if perms >= req {
        update.update_project(&mut project);
        update_project(&mut conn, &mut project).await?;
        Ok(web::Json(project.into()))
    } else {
        Err(unauthorized("Not Authorized to update project"))
    }
}

async fn help_get_project_permissions(
    user: &AuthenticatedUser,
    proj: &ProjectModel,
    conn: &mut PgConnection,
) -> Result<Permissions> {
    if user.is_admin {
        Ok(Permissions::Admin)
    } else if let Some(perms) = proj.get_permissions(user.id) {
        Ok(perms)
    } else {
        get_project_permission(conn, user.id, proj.id).await
    }
}


async fn create_column_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<CreateProjectColumnMessage>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<ProjectColumnMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let project_id = path.into_inner();
    let mut conn = pool.acquire().await?;
    if user.is_admin || get_project_permission(&mut conn, user.id, project_id).await? >= Permissions::Editor {
        let mut new_column = msg.0.to_model(project_id);
        create_column(&mut conn, &mut new_column).await?;
        Ok(web::Json(new_column.into()))
    } else {
        Err(unauthorized("Insufficient permissions"))
    }
}

async fn create_label_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<CreateLabelMessage>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<LabelMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let project_id = path.into_inner();
    let mut conn = pool.acquire().await?;
    if user.is_admin || get_project_permission(&mut conn, user.id, project_id).await? >= Permissions::Editor {
        let mut new_label = msg.0.to_model(project_id);
        create_label(&mut conn, &mut new_label).await?;
        Ok(web::Json(new_label.into()))
    } else {
        Err(unauthorized("Insufficient permissions"))
    }
}

async fn update_column_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<UpdateProjectColumnMessage>,
    req: HttpRequest,
    path: web::Path<(Uuid,Uuid)>,
) -> Result<web::Json<ProjectColumnMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let (project_id, column_id) = path.into_inner();
    let mut conn = pool.acquire().await?;
    if user.is_admin || get_project_permission(&mut conn, user.id, project_id).await? >= Permissions::Editor {
        let mut col = get_column(&mut conn, column_id).await?.ok_or_else(|| not_found("Column not found"))?;
        msg.0.update_model(&mut col);
        update_column(&mut conn, &col).await?;
        Ok(web::Json(col.into()))
    } else {
        Err(unauthorized("Insufficient permissions"))
    }
}

async fn update_label_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    msg: web::Json<UpdateLabelMessage>,
    req: HttpRequest,
    path: web::Path<(Uuid,Uuid)>,
) -> Result<web::Json<LabelMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let (project_id, label_id) = path.into_inner();
    let mut conn = pool.acquire().await?;
    if user.is_admin || get_project_permission(&mut conn, user.id, project_id).await? >= Permissions::Editor {
        let mut col = get_label(&mut conn, label_id).await?.ok_or_else(|| not_found("label not found"))?;
        msg.0.update_model(&mut col);
        update_label(&mut conn, &col).await?;
        Ok(web::Json(col.into()))
    } else {
        Err(unauthorized("Insufficient permissions"))
    }
}


async fn delete_column_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<(Uuid,Uuid)>,
) -> Result<web::Json<SuccessMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let (project_id, column_id) = path.into_inner();
    let mut conn = pool.acquire().await?;
    if user.is_admin || get_project_permission(&mut conn, user.id, project_id).await? >= Permissions::Editor {
        delete_column(&mut conn, column_id).await?;
        Ok(web::Json(SuccessMessage::new(true)))
    } else {
        Err(unauthorized("Insufficient permissions"))
    }
}

async fn delete_label_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<(Uuid,Uuid)>,
) -> Result<web::Json<SuccessMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let (project_id, label_id) = path.into_inner();
    let mut conn = pool.acquire().await?;
    if user.is_admin || get_project_permission(&mut conn, user.id, project_id).await? >= Permissions::Editor {
        delete_label(&mut conn, label_id).await?;
        Ok(web::Json(SuccessMessage::new(true)))
    } else {
        Err(unauthorized("Insufficient permissions"))
    }
}