use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::models::UserModel;
use crate::data::project_data::{create_project, delete_project, get_project, get_projects, list_projects};
use crate::error::{not_found, unauthorized, Result};
use crate::messages::project_messages::{CreateProjectMessage, ProjectMessage};
use crate::util::{Page, Pagination};

pub async fn handle_create_project(
    msg: CreateProjectMessage,
    user_id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<ProjectMessage> {
    let mut new_project = msg.to_model(user_id);
    create_project(&mut *pool.acquire().await?, &mut new_project).await?;
    Ok(new_project.into())
}


pub async fn handle_get_project(
    user: &UserModel,
    project_id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<ProjectMessage> {
    let proj: ProjectMessage = get_project(&mut *pool.acquire().await?, project_id).await?.map(Into::into).ok_or_else(|| not_found("Project Not Found"))?;
    if user.is_admin || proj.owner_id == user.id {
        Ok(proj)
    } else {
        Err(unauthorized("Not allowed"))
    }
}

pub async fn handle_get_projects(
    pag: Pagination,
    user: &UserModel,
    pool: &Pool<Postgres>,
) -> Result<Page<ProjectMessage>> {
    let (est,projects) = if user.is_admin {
        list_projects(&mut *pool.acquire().await?, pag).await?
    } else {
        get_projects(&mut *pool.acquire().await?, user.id, pag).await?
    };
    Ok(Page::new(projects.into_iter().map(Into::into).collect(), pag, est))
}

pub async fn handle_delete_project(
    project_id: Uuid,
    user: &UserModel,
    pool: &Pool<Postgres>,
) -> Result<()> {
    let project = get_project(&mut *pool.acquire().await?, project_id).await?;
    if let Some(proj) = project {
        if user.is_admin || user.id == proj.owner_id {
            delete_project(&mut *pool.acquire().await?, project_id).await?;
        }
    }
    Ok(())
}