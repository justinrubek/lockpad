use lockpad_ulid::Ulid;
use sqlx::types::Uuid;

pub mod application;
pub mod entity;
pub mod error;
pub mod user;

#[derive(Debug)]
pub struct User {
    pub user_id: Option<lockpad_ulid::Ulid>,
    // pub user_id: Option<sqlx::types::Uuid>,
    pub name: String,
}

pub async fn temp_example() {
    use sqlx::postgres::PgPool;
    use std::env;

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // let query = sqlx::query!(r#"SELECT user_id::TEXT as "user_id: Ulid" FROM users"#)
    // let query = sqlx::query!(r#"SELECT user_id::UUID as "user_id: Uuid" FROM users"#)

    let insert = sqlx::query!(
        r#"INSERT INTO users (name, secret) VALUES ($1, $2)"#,
        "test",
        "secret"
    )
    .execute(&pool)
    .await
    .unwrap();

    let generated_id = Ulid::generate();
    let generated_uuid = Uuid::from(generated_id);
    let generated_string = generated_id.to_string();
    let uuid_string = generated_uuid.to_string();
    let second_insert = sqlx::query!(
        r#"
        INSERT INTO 
            users(user_id, name, secret)
        SELECT user_id::uuid, name, secret
        FROM(
            VALUES($1, $2, $3)
        ) AS data(user_id, name, secret)
        "#,
        uuid_string,
        "generated-test",
        "secret"
    )
    .execute(&pool)
    .await
    .unwrap();

    let query = sqlx::query!(r#"SELECT user_id::UUID as "user_id: Ulid" FROM users"#)
        // let query = sqlx::query_as!(User, r#"SELECT name, user_id::UUID as "user_id: Ulid" FROM users"#)
        .fetch_all(&pool)
        .await
        .unwrap();

    query.iter().for_each(|user| {
        println!("{:?}", user);
        let id = user.user_id.unwrap();
        println!("{:?}", id);
    });
}
