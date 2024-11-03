use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::data::project_data::create_project;
use crate::error::Result;
use crate::messages::project_messages::{CreateProjectMessage, ProjectMessage};

pub async fn handle_create_project(
    msg: CreateProjectMessage,
    user_id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<ProjectMessage> {
    let mut new_project = msg.to_model(user_id);
    create_project(&mut *pool.acquire().await?, &mut new_project).await?;
    Ok(new_project.into())
}
