use crate::{error::Result, util::Pagination};
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


pub async fn get_project(conn: &mut PgConnection, id: Uuid) -> Result<Option<ProjectModel>> {
    let proj = sqlx::query_as(
        "Select * FROM projects where id=$1",
    )
    .bind(id)
    .fetch_optional(conn)
    .await?;
    Ok(proj)
}

pub async fn get_projects(conn: &mut PgConnection, owner_id: Uuid, pag: Pagination) -> Result<(i32, Vec<ProjectModel>)> {
    let projects= sqlx::query_as(
        "select * from projects where owner_id=$1 order by created ASC limit $2 offset $3",
    )
    .bind(owner_id)
    .bind(pag.count)
    .bind(pag.count * (pag.page - 1))
    .fetch_all(conn)
    .await?;
    let estimate = 42;
    Ok((estimate,projects))
}

pub async fn list_projects(conn: &mut PgConnection, pag: Pagination) -> Result<(i32, Vec<ProjectModel>)> {
    let projects = sqlx::query_as(
        "select * from projects order by created ASC limit $1 offset $2",
    )
    .bind(pag.count)
    .bind(pag.count * (pag.page - 1))
    .fetch_all(conn)
    .await?;
    let estimate = 42;
    Ok((estimate,projects))
}

pub async fn update_project(conn: &mut PgConnection, proj: &mut ProjectModel) -> Result<()> {
    sqlx::query(
        "update projects set name=$1, public=$2, owner_id=$3 where id=$4",
    )
    .bind(&proj.name)
    .bind(&proj.public)
    .bind(proj.owner_id)
    .bind(proj.id)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn delete_project(conn: &mut PgConnection, id: Uuid) -> Result<()> {
    sqlx::query(
        "delete from projects where id=$1",
    )
    .bind(id)
    .execute(conn)
    .await?;
    Ok(())
}