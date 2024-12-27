use actix_web::{web, HttpRequest};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::models::{AuthenticatedUser, Permissions};
use crate::data::project_data::get_project_permission;
use crate::data::task_data::{
    add_task_label, create_task, get_task, get_task_labels, remove_task_label, update_task,
};
use crate::error::{not_found, unauthorized, Result};
use crate::handler::user_handler::handle_token_update;
use crate::messages::general_messages::SuccessMessage;
use crate::messages::task_massages::{CreateOrUpdateTaskMessage, TaskMessage};

pub fn register_task_routes(cfg: &mut web::ServiceConfig) {
    // cfg.service(web::scope("/user").route("/new", web::post().to(create_user)));
    cfg.service(
        web::scope("/tasks")
            .route("", web::post().to(create_task_route))
            .route("/{task_id}", web::post().to(update_task_route))
            .route("/{task_id}", web::delete().to(delete_task_route))
            .route("/{task_id}", web::get().to(get_task_route)),
    );
}

async fn create_task_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    mut msg: web::Json<CreateOrUpdateTaskMessage>,
) -> Result<web::Json<TaskMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;

    let labels = msg.0.labels.take().unwrap_or_default();
    let mut task = msg.0.to_model();
    let mut t = pool.begin().await?;
    let perm = get_project_permission(&mut t, user.id, task.project_id).await?;

    if perm >= Permissions::Editor {
        create_task(&mut t, &mut task).await?;
        for id in &labels {
            add_task_label(&mut t, task.id, *id).await?;
        }
        let mut rmsg: TaskMessage = task.into();
        rmsg.labels = Some(labels);
        Ok(web::Json(rmsg))
    } else {
        Err(unauthorized("You shall not pass"))
    }
}

async fn update_task_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    mut msg: web::Json<CreateOrUpdateTaskMessage>,
    path: web::Path<Uuid>,
) -> Result<web::Json<TaskMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let mut t = pool.begin().await?;
    let task_id = path.into_inner();
    let new_labels = msg.0.labels.take().unwrap_or_default();
    let mut task = get_task(&mut t, task_id)
        .await?
        .ok_or_else(|| not_found("Task not found"))?;
    msg.0.update_model(&mut task);
    let perm = get_project_permission(&mut t, user.id, task.project_id).await?;

    let old_labels = get_task_labels(&mut t, task.id).await?;
    if perm >= Permissions::Editor {
        update_task(&mut t, &mut task).await?;
        for new_id in new_labels
            .iter()
            .filter(|x| !old_labels.iter().any(|y| y.id == **x))
        {
            add_task_label(&mut t, task.id, *new_id).await?;
        }

        for old_label in old_labels
            .iter()
            .filter(|x| !new_labels.iter().any(|y| x.id == *y))
        {
            remove_task_label(&mut t, task.id, old_label.id).await?;
        }
        let mut rmsg: TaskMessage = task.into();
        rmsg.labels = Some(new_labels);
        Ok(web::Json(rmsg))
    } else {
        Err(unauthorized("You shall not pass"))
    }
}

async fn delete_task_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<SuccessMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let task_id = path.into_inner();
    let mut conn = pool.acquire().await?;
    let task = get_task(&mut conn, task_id)
        .await?
        .ok_or_else(|| not_found("task not found"))?;
    let perms = get_project_permission(&mut conn, user.id, task.project_id).await?;
    if perms >= Permissions::Editor {
        Ok(web::Json(SuccessMessage::new(true)))
    } else {
        Err(unauthorized("You cannot do that"))
    }
}

async fn get_task_route(
    user: AuthenticatedUser,
    pool: web::Data<Pool<Postgres>>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<web::Json<TaskMessage>> {
    handle_token_update(&pool, &user, &user.token, &req).await?;
    let task_id = path.into_inner();
    let mut conn = pool.acquire().await?;
    let task = get_task(&mut conn, task_id)
        .await?
        .ok_or_else(|| not_found("task not found"))?;
    let perms = get_project_permission(&mut conn, user.id, task.project_id).await?;
    if perms >= Permissions::Reader {
        let labels = get_task_labels(&mut conn, task.id)
            .await?
            .into_iter()
            .map(|x| x.id)
            .collect();
        let mut rmsg: TaskMessage = task.into();
        rmsg.labels = Some(labels);
        Ok(web::Json(rmsg))
    } else {
        Err(unauthorized("You cannot do that"))
    }
}
