use crate::error::{Error, Result};
use lockpad_ulid::Ulid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub user_id: Ulid,
    pub identifier: String,
    #[serde(skip_serializing)]
    pub secret: String,
}

impl User {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn by_id(pool: &sqlx::pool::Pool<sqlx::Postgres>, id: &Ulid) -> Result<Option<Self>> {
        // let query = sqlx::query!(r#"SELECT user_id::UUID as "user_id: Ulid" FROM users"#)
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                user_id as "user_id: Ulid", identifier, secret
            FROM 
                users
            WHERE 
                user_id = $1
            "#,
            id as _,
        )
        .fetch_one(pool)
        .await?;

        Ok(Some(user))
    }

    pub async fn by_identifier(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        identifier: &str,
    ) -> Result<Option<Self>> {
        let query = sqlx::query!(
            r#"
            SELECT
                user_id::uuid as "user_id: Ulid", identifier, secret
            FROM 
                users
            WHERE 
                identifier = $1
            "#,
            identifier,
        );

        let user = query.fetch_one(pool).await?;

        let user = match user.user_id {
            Some(user_id) => Some(User {
                user_id,
                identifier: user.identifier,
                secret: user.secret,
            }),
            None => None,
        };

        Ok(user)
    }

    pub async fn create(&self, pool: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO 
                users(user_id, identifier, secret)
            SELECT 
                user_id::uuid, identifier, secret
            FROM(
                VALUES($1, $2, $3)
            ) AS data(user_id, identifier, secret)
            "#,
            self.user_id.queryable(),
            self.identifier,
            self.secret,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn query(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        _pagination: crate::Pagination,
    ) -> Result<(Vec<Self>, crate::Pagination)> {
        // TODO: Implement pagination for querying
        let query = sqlx::query!(
            r#"
            SELECT
                user_id::uuid as "user_id: Ulid", identifier, secret
            FROM 
                users
            "#,
        );

        let users = query.fetch_all(pool).await?;

        let users = users
            .into_iter()
            .filter_map(|user| match user.user_id {
                Some(user_id) => Some(User {
                    user_id,
                    identifier: user.identifier,
                    secret: user.secret,
                }),
                None => None,
            })
            .collect::<Vec<_>>();

        let pagination = crate::Pagination {
            last_key: users.last().map(|user| user.user_id),
            count: users.len(),
        };

        Ok((users, pagination))
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    identifier: Option<String>,
    secret: Option<String>,
}

impl Builder {
    pub fn identifier(mut self, identifier: String) -> Self {
        self.identifier = Some(identifier);
        self
    }

    pub fn secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = User;

    fn build(self) -> Result<Self::Item> {
        let identifier = self
            .identifier
            .ok_or_else(|| Error::ModelFieldsMissing("identifier"))?;
        let secret = self
            .secret
            .ok_or_else(|| Error::ModelFieldsMissing("secret"))?;

        Ok(User {
            user_id: Ulid::generate(),
            identifier,
            secret,
        })
    }
}
