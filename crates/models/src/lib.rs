pub mod application;
pub mod entity;
pub mod error;
pub mod user;

pub async fn temp_example() {
    use sqlx::postgres::PgPool;
    use std::env;

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let _query = sqlx::query!("SELECT name FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
}
