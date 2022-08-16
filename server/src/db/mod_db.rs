
use crate::{DBPool, DBConn, error, error::Error::*};
use std::{time, fs, str::FromStr};
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use tokio_postgres::{NoTls, Config, Error};


pub type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SEC: u64 = 15;
const SQL_INIT: &str = "./db.sql";

pub async fn db_init(pool: &DBPool) -> Result<()> {
    let sql_file = fs::read_to_string(SQL_INIT)?;
    let conn = get_db_conn(pool).await?;
    conn.batch_execute(sql_file.as_str()).await.map_err(DBInitError)?;
    Ok(())
}


pub async fn get_db_conn(pool: &DBPool) -> Result<DBConn> {
    pool.get().await.map_err(DBPoolError)
}


pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let cfg = Config::from_str("postgresql://root:secret@localhost:5432/fullstack_app?sslmode=disable")?;
    let manager = PgConnectionManager::new(cfg, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(time::Duration::from_secs(DB_POOL_TIMEOUT_SEC)))
        .build(manager)
    )
}