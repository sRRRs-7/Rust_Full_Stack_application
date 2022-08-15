type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN = 32;
const DB_POOL_MAX_IDLE = 8;
const DB_POOL_TIMEOUT_SEC = 15;
const DB_INIT = "./db.sql";

