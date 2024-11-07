use sqlx::types::Uuid;
use sqlx::PgConnection;

use crate::data::models::UserModel;
use crate::error::Result;
use crate::util::{from_pg_rows, Pagination};

use super::models::UserSessionModel;

pub async fn create_user(conn: &mut PgConnection, user: &mut UserModel) -> Result<()> {
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (username, password_hash, enabled, is_admin) Values($1, $2, true, $3) returning id",
    )
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(user.is_admin)
    .fetch_one(conn)
    .await?;
    user.id = id;
    Ok(())
}

pub async fn update_user(conn: &mut PgConnection, user: &UserModel) -> Result<()> {
    sqlx::query("UPDATE users SET password_hash=$1, username=$2, is_admin=$3 WHERE id=$4")
        .bind(&user.password_hash)
        .bind(&user.username)
        .bind(user.is_admin)
        .bind(user.id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn get_users(conn: &mut PgConnection, pag: Pagination) -> Result<(i64, Vec<UserModel>)> {
    let r = sqlx::query("Select *, count(*) over() as full_count from users order by created ASC limit $1 offset $2")
        .bind(pag.count)
        .bind(pag.count * (pag.page - 1))
        .fetch_all(&mut *conn)
        .await?;
    from_pg_rows(&r, "full_count")
}

pub async fn get_user_from_session(
    conn: &mut PgConnection,
    session_token: &str,
) -> Result<Option<UserModel>> {
    let user = sqlx::query_as("select u.id, u.username, u.password_hash, u.is_admin from users u join user_sessions s on s.user_id=u.id where token=$1")
        .bind(session_token)
        .fetch_optional(conn)
        .await?;
    Ok(user)
}

pub async fn get_user_by_name(
    conn: &mut PgConnection,
    username: &str,
) -> Result<Option<UserModel>> {
    let user = sqlx::query_as(
        "select u.id, u.username, u.password_hash, u.is_admin from users u where username=$1",
    )
    .bind(username)
    .fetch_optional(conn)
    .await?;
    Ok(user)
}

pub async fn get_user_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Option<UserModel>> {
    let user = sqlx::query_as(
        "select u.id, u.username, u.password_hash, u.is_admin from users u where id=$1",
    )
    .bind(id)
    .fetch_optional(conn)
    .await?;
    Ok(user)
}

pub async fn create_user_session(conn: &mut PgConnection, session: UserSessionModel) -> Result<()> {
    sqlx::query(
        "insert into user_sessions (token, user_agent, ip_addr, user_id) values($1, $2, $3, $4)",
    )
    .bind(&session.token)
    .bind(&session.user_agent)
    .bind(&session.ip_addr)
    .bind(session.user_id)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn update_user_session(conn: &mut PgConnection, session: UserSessionModel) -> Result<()> {
    sqlx::query("update user_sessions set user_agent=$1, ip_addr=$2 where token=$3")
        .bind(&session.user_agent)
        .bind(&session.ip_addr)
        .bind(&session.token)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn check_username_available(conn: &mut PgConnection, username: &str) -> Result<bool> {
    Ok(sqlx::query("select username from users where username=$1")
        .bind(username)
        .fetch_optional(conn)
        .await?
        .is_none())
}

pub async fn delete_user(conn: &mut PgConnection, id: Uuid) -> Result<()> {
    sqlx::query("delete from users where id=$1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}
