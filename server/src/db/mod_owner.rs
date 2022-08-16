use crate::{DBPool, error::Error::DBQueryError};
use super::mod_db::{Result, get_db_conn,};
use common::*;
use mobc_postgres::tokio_postgres::Row;

// owner CRUD

pub const OWNER_TABLE: &str = "owner";
const OWNER_FIELD: &str = "id, name";


pub async fn get_all(pool: &DBPool) -> Result<Vec<Owner>> {
    let conn = get_db_conn(pool).await?;
    let query = format!("SELECT {} FROM {}", OWNER_FIELD, OWNER_TABLE);
    let rows = conn.query(query.as_str(), &[]).await.map_err(DBQueryError)?;
    Ok(rows.iter().map(|r| {row_to_owner(&r)}).collect())
}


pub async fn get_one(pool: &DBPool, id: i32) -> Result<Owner> {
    let conn = get_db_conn(pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1", OWNER_FIELD, OWNER_TABLE );
    let row = conn.query_one(query.as_str(), &[&id]).await.map_err(DBQueryError)?;
    Ok(row_to_owner(&row))
}


pub async fn insert(pool: &DBPool, body: OwnerRequest) -> Result<Owner> {
    let conn = get_db_conn(pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", OWNER_TABLE);
    let row = conn.query_one(query.as_str(), &[&body.name]).await.map_err(DBQueryError)?;
    Ok(row_to_owner(&row))
}


fn row_to_owner(row: &Row) -> Owner {
    let id = row.get(0);
    let name = row.get(1);
    Owner {id, name}
}