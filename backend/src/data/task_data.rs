use sqlx::PgConnection;
use uuid::Uuid;

use crate::error::Result;

use super::models::{LabelModel, TaskModel};

pub async fn create_task(conn: &mut PgConnection, model: &mut TaskModel) -> Result<()> {
    let id = sqlx::query_scalar(
        "insert into tasks (
            title,
            parent_id,
            column_id,
            creator_id,
            deadline,
            estimation,
            project_id,
            state_id,
            task_type,
            assignee_id
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        where id=$11 returning id",
    )
    .bind(&model.title)
    .bind(model.parent_id)
    .bind(model.column_id)
    .bind(model.creator_id)
    .bind(model.deadline)
    .bind(model.estimation)
    .bind(model.project_id)
    .bind(model.state_id)
    .bind(model.task_type)
    .bind(model.assignee_id)
    .bind(model.id)
    .fetch_one(conn)
    .await?;
    model.id = id;
    Ok(())
}

pub async fn update_task(conn: &mut PgConnection, model: &mut TaskModel) -> Result<()> {
    sqlx::query(
        "update tasks set
            title=$1,
            parent_id=$2,
            column_id=$3,
            creator_id=$4,
            deadline=$5,
            estimation=$6,
            project_id=$7,
            state_id=$8,
            task_type=$9,
            assignee_id=$10
        where id=$11",
    )
    .bind(&model.title)
    .bind(model.parent_id)
    .bind(model.column_id)
    .bind(model.creator_id)
    .bind(model.deadline)
    .bind(model.estimation)
    .bind(model.project_id)
    .bind(model.state_id)
    .bind(model.task_type)
    .bind(model.assignee_id)
    .bind(model.id)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn get_task(conn: &mut PgConnection, task_id: Uuid) -> Result<Option<TaskModel>> {
    let task = sqlx::query_as("select * from tasks where id=$1")
        .bind(task_id)
        .fetch_optional(conn)
        .await?;
    Ok(task)
}

pub async fn delete_task(conn: &mut PgConnection, task_id: Uuid) -> Result<()> {
    sqlx::query("delete from tasks where id=$1")
        .bind(task_id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_tasks_by_project(
    conn: &mut PgConnection,
    project_id: Uuid,
) -> Result<Vec<TaskModel>> {
    let tasks = sqlx::query_as("select * from tasks where project_id=$1")
        .bind(project_id)
        .fetch_all(conn)
        .await?;
    Ok(tasks)
}

pub async fn add_task_label(conn: &mut PgConnection, task_id: Uuid, label_id: Uuid) -> Result<()> {
    sqlx::query("insert into labels_tasks (task_id, label_id) values($1, $2)")
        .bind(task_id)
        .bind(label_id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn remove_task_label(
    conn: &mut PgConnection,
    task_id: Uuid,
    label_id: Uuid,
) -> Result<()> {
    sqlx::query("delete from labels_tasls where task_id=$1 and label_id=$2)")
        .bind(task_id)
        .bind(label_id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_task_labels(conn: &mut PgConnection, task_id: Uuid) -> Result<Vec<LabelModel>> {
    let labels = sqlx::query_as(
        "select * from labels l join labels_tasks lt on lt.label_id=l.id where task_id=$1",
    )
    .bind(task_id)
    .fetch_all(conn)
    .await?;
    Ok(labels)
}
// pub async fn get_tasks_by_column(conn: &mut PgConnection, column_id: Uuid) -> Result<Vec<TaskModel>> {
//     let tasks = sqlx::query_as("select * from tasks where column_id=$1")
//     .bind(column_id)
//     .fetch_all(conn).await?;
//     Ok(tasks)
// }
