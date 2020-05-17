use crate::models::{SlimUser, User};
use sqlx::MySqlPool;
use sqlx::Pool;
use sqlx::SqlitePool;

pub struct Context {
    pub pool: SqlitePool,
    pub user: Option<User>,
}

// impl Context<MySqlPool> {
//     pub async fn new(pool: MySqlPool, user: SlimUser) -> Self {
//         use sqlx::mysql::MySqlQueryAs;
//         let user = match user.id {
//             Some(id) => {
//                 let result = sqlx::query_as::<_, User>("select * from users where id = $1")
//                     .bind(id)
//                     .fetch_one(&pool)
//                     .await;
//                 result.ok()
//             }
//             None => None,
//         };
//         Self { pool, user }
//     }
// }

impl Context {
    pub async fn new(pool: SqlitePool, user: SlimUser) -> Self {
        use sqlx::sqlite::SqliteQueryAs;
        let user = match user.id {
            Some(id) => {
                let result = sqlx::query_as::<_, User>("select * from users where id = $1")
                    .bind(id)
                    .fetch_one(&pool)
                    .await;
                result.ok()
            }
            None => None,
        };
        Self { pool, user }
    }
}
