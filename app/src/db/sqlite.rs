use sqlx::SqlitePool;

pub async fn sqlite_pool() -> SqlitePool {
    SqlitePool::new("sqlite::memory:")
        .await
        .expect("Failed to create Sqlite Pool")
}
