
use crate::{DBPool, error::Error::DBQueryError};
use super::mod_db::{Result, get_db_conn,};
use common::*;
use mobc_postgres::tokio_postgres::Row;

// owner CRUD

pub const DEVICE_TABLE: &str = "device";
const DEVICE_FIELD: &str = "id, product, owner_id, maker, feature";


pub async fn get_all(pool: &DBPool, owner_id: i32) -> Result<Vec<Device>> {
    let conn = get_db_conn(pool).await?;
    let query = format!("SELECT {} FROM {} WHERE owner_id = $1", DEVICE_FIELD, DEVICE_TABLE);
    let rows = conn.query(query.as_str(), &[&owner_id]).await.map_err(DBQueryError)?;
    Ok(rows.iter().map(|r| {row_to_device(&r)}).collect())
}


pub async fn insert(pool: &DBPool, body: DeviceRequest) -> Result<Device> {
    let conn = get_db_conn(pool).await?;
    let query = format!("INSERT INTO {} (product, owner_id, maker, feature) VALUES ($1, $2, $3, $4) RETURNING *", DEVICE_TABLE);
    let row = conn.query_one(query.as_str(), &[&body.product, &body.owner_id, &body.maker, &body.feature]).await.map_err(DBQueryError)?;
    Ok(row_to_device(&row))
}


pub async fn delete(pool: &DBPool, id: i32) -> Result<u64> {
    let conn = get_db_conn(pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", DEVICE_TABLE);
    conn.execute(query.as_str(), &[&id]).await.map_err(DBQueryError)
}


fn row_to_device(row: &Row) -> Device {
    let id = row.get(0);
    let product = row.get(1);
    let owner_id = row.get(2);
    let maker = row.get(3);
    let feature = row.get(4);
    Device { id, product, owner_id, maker, feature }
}