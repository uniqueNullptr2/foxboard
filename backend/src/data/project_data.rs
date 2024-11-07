use super::models::{LabelModel, Permissions, ProjectColumnModel, ProjectModel, StateModel};
use crate::{
    error::Result,
    util::{from_pg_rows, Pagination},
};
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn create_project(conn: &mut PgConnection, proj: &mut ProjectModel) -> Result<()> {
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO projects (name, public, owner_id) values($1, $2, $3) returning id",
    )
    .bind(&proj.name)
    .bind(proj.public)
    .bind(proj.owner_id)
    .fetch_one(conn)
    .await?;
    proj.id = id;
    Ok(())
}

pub async fn get_project(conn: &mut PgConnection, id: Uuid) -> Result<Option<ProjectModel>> {
    let proj = sqlx::query_as("Select * FROM projects where id=$1")
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(proj)
}

pub async fn get_projects(
    conn: &mut PgConnection,
    owner_id: Uuid,
    pag: Pagination,
) -> Result<(i64, Vec<ProjectModel>)> {
    let r= sqlx::query(
        "select *, count(*) over() as full_count from projects where owner_id=$1 or public=true order by created ASC limit $2 offset $3",
    )
    .bind(owner_id)
    .bind(pag.count)
    .bind(pag.count * (pag.page - 1))
    .fetch_all(conn)
    .await?;
    from_pg_rows(&r, "full_count")
}

pub async fn list_projects(
    conn: &mut PgConnection,
    pag: Pagination,
) -> Result<(i64, Vec<ProjectModel>)> {
    let r= sqlx::query(
        "select *, count(*) over() as full_count from projects order by created ASC limit $1 offset $2",
    )
    .bind(pag.count)
    .bind(pag.count * (pag.page - 1))
    .fetch_all(conn)
    .await?;
    from_pg_rows(&r, "full_count")
}

pub async fn update_project(conn: &mut PgConnection, proj: &mut ProjectModel) -> Result<()> {
    sqlx::query("update projects set name=$1, public=$2, owner_id=$3 where id=$4")
        .bind(&proj.name)
        .bind(proj.public)
        .bind(proj.owner_id)
        .bind(proj.id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn delete_project(conn: &mut PgConnection, id: Uuid) -> Result<()> {
    sqlx::query("delete from projects where id=$1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_project_permission(
    conn: &mut PgConnection,
    user_id: Uuid,
    project_id: Uuid,
) -> Result<Permissions> {
    Ok(
        sqlx::query_as("Select case when p.owner_id=$1 then 1 else pp.perm end as perms from projects p left join project_permissions pp on pp.project_id=p.id and pp.user_id=$1 where p.id=$2")
            .bind(user_id)
            .bind(project_id)
            .fetch_optional(conn)
            .await?
            .unwrap_or_default(),
    )
}

pub async fn create_column(conn: &mut PgConnection, col: &mut ProjectColumnModel) -> Result<()> {
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO project_columns (name, card_limit, index, project_id) values($1, $2, $3, $4) returning id",
    )
    .bind(&col.name)
    .bind(col.card_limit)
    .bind(col.index)
    .bind(col.project_id)
    .fetch_one(conn)
    .await?;
    col.id = id;
    Ok(())
}

pub async fn create_label(conn: &mut PgConnection, lab: &mut LabelModel) -> Result<()> {
    let id: Uuid =
        sqlx::query_scalar("INSERT INTO labels (name, project_id) values($1, $2) returning id")
            .bind(&lab.name)
            .bind(lab.project_id)
            .fetch_one(conn)
            .await?;
    lab.id = id;
    Ok(())
}

pub async fn get_column(conn: &mut PgConnection, id: Uuid) -> Result<Option<ProjectColumnModel>> {
    let col = sqlx::query_as("Select * FROM project_columns where id=$1")
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(col)
}

pub async fn get_label(conn: &mut PgConnection, id: Uuid) -> Result<Option<LabelModel>> {
    let lab = sqlx::query_as("Select * FROM labels where id=$1")
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(lab)
}

pub async fn update_column(conn: &mut PgConnection, col: &ProjectColumnModel) -> Result<()> {
    sqlx::query("UPDATE project_columns SET name=$1, index=$2, card_limit=$3 where id=$4")
        .bind(&col.name)
        .bind(col.index)
        .bind(col.card_limit)
        .bind(col.id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn update_label(conn: &mut PgConnection, lab: &LabelModel) -> Result<()> {
    sqlx::query("UPDATE labels SET name=$1 where id=$2")
        .bind(&lab.name)
        .bind(lab.id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn delete_column(conn: &mut PgConnection, column_id: Uuid) -> Result<()> {
    sqlx::query("delete from project_columns where id=$1")
        .bind(column_id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn delete_label(conn: &mut PgConnection, label_id: Uuid) -> Result<()> {
    sqlx::query("delete from labels where id=$1")
        .bind(label_id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_columns(
    conn: &mut PgConnection,
    project_id: Uuid,
) -> Result<Vec<ProjectColumnModel>> {
    let columns = sqlx::query_as(
        "select * from project_columns where project_id=$1 order by index, created ASC",
    )
    .bind(project_id)
    .fetch_all(conn)
    .await?;
    Ok(columns)
}

pub async fn get_labels(conn: &mut PgConnection, project_id: Uuid) -> Result<Vec<LabelModel>> {
    let labels = sqlx::query_as("select * from labels where project_id=$1 order by created asc")
        .bind(project_id)
        .fetch_all(conn)
        .await?;
    Ok(labels)
}

pub async fn create_state(conn: &mut PgConnection, state: &mut StateModel) -> Result<()> {
    let id: Uuid =
        sqlx::query_scalar("INSERT INTO states (name, project_id) values($1, $2) returning id")
            .bind(&state.name)
            .bind(state.project_id)
            .fetch_one(conn)
            .await?;
    state.id = id;
    Ok(())
}
pub async fn get_state(conn: &mut PgConnection, id: Uuid) -> Result<Option<StateModel>> {
    let state = sqlx::query_as("Select * FROM states where id=$1")
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(state)
}
pub async fn update_state(conn: &mut PgConnection, state: &StateModel) -> Result<()> {
    sqlx::query("UPDATE states SET name=$1 where id=$2")
        .bind(&state.name)
        .bind(state.id)
        .execute(conn)
        .await?;
    Ok(())
}
pub async fn delete_state(conn: &mut PgConnection, state_id: Uuid) -> Result<()> {
    sqlx::query("delete from states where id=$1")
        .bind(state_id)
        .execute(conn)
        .await?;
    Ok(())
}
pub async fn get_states(conn: &mut PgConnection, project_id: Uuid) -> Result<Vec<StateModel>> {
    let states = sqlx::query_as("select * from states where project_id=$1 order by created asc")
        .bind(project_id)
        .fetch_all(conn)
        .await?;
    Ok(states)
}
