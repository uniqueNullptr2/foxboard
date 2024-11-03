use crate::error::Result;
use sqlx::PgConnection;
use uuid::Uuid;

use super::models::ProjectModel;

pub async fn create_project(conn: &mut PgConnection, proj: &mut ProjectModel) -> Result<()> {
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO projects (name, public, owner_id) values($1, $2, $3) returning id",
    )
    .bind(&proj.name)
    .bind(&proj.public)
    .bind(proj.owner_id)
    .fetch_one(conn)
    .await?;
    proj.id = id;
    Ok(())
}
