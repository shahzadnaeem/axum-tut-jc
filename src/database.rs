use sqlx::sqlite::SqlitePool;

pub const DB_SQLITE: &str = "db/db.sqlite";

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Self {
        // TODO: Use Result?
        let pool = SqlitePool::connect(DB_SQLITE).await.unwrap();

        Database { pool }
    }
}
