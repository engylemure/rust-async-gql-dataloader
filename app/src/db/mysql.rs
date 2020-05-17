use crate::utils::env::ENV;
use sqlx::MySqlPool;

pub async fn mysql_pool() -> MySqlPool {
    MySqlPool::new(&ENV.database_url)
        .await
        .expect("Failed to create MySql Pool")
}
