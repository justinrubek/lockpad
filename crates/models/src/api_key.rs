use crate::error::{Error, Result};
use lockpad_ulid::Ulid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub api_key_id: Ulid,
    pub owner_id: Ulid,
    pub name: String,
    pub secret: String,
}

impl ApiKey {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn by_id(pool: &sqlx::pool::Pool<sqlx::Postgres>, id: &Ulid) -> Result<Option<Self>> {
        let query = sqlx::query!(
            r#"
            SELECT
                api_key_id::uuid as "api_key_id: Ulid",
                owner_id::uuid as "owner_id: Ulid",
                name,
                secret
            FROM 
                api_keys
            WHERE 
                api_key_id::uuid = $1
            "#,
            id.to_sqlx_uuid()
        );

        let api_key = query.fetch_one(pool).await?;
        match (api_key.api_key_id, api_key.owner_id) {
            (Some(api_key_id), Some(owner_id)) => {
                let api_key = ApiKey {
                    api_key_id,
                    owner_id,
                    name: api_key.name,
                    secret: api_key.secret,
                };

                Ok(Some(api_key))
            }
            _ => Ok(None),
        }
    }

    pub async fn by_owner_id(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        owner_id: &Ulid,
    ) -> Result<Option<Self>> {
        let api_key = sqlx::query_as!(
            ApiKey,
            r#"
            SELECT
                api_key_id as "api_key_id: Ulid", owner_id as "owner_id: Ulid", name, secret
            FROM 
                api_keys
            WHERE 
                owner_id = $1
            "#,
            owner_id as _,
        )
        .fetch_one(pool)
        .await?;

        Ok(Some(api_key))
    }

    pub async fn create(&self, pool: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO 
                api_keys(api_key_id, owner_id, name, secret)
            SELECT 
                api_key_id::uuid, owner_id::uuid, name, secret
            FROM(
                VALUES(
                    $1, $2, $3, $4
                )
            ) AS data(api_key_id, owner_id, name, secret)
            "#,
        )
        .bind(self.api_key_id.queryable())
        .bind(self.owner_id.queryable())
        .bind(&self.name)
        .bind(&self.secret)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn query(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        _owner_id: Ulid,
        _pagination: crate::Pagination,
    ) -> Result<(Vec<Self>, crate::Pagination)> {
        // TODO: Implement pagination for querying
        let api_keys = sqlx::query!(
            r#"
            SELECT
                api_key_id::uuid as "api_key_id: Ulid",
                owner_id::uuid as "owner_id: Ulid",
                name, secret
            FROM 
                api_keys
            "#,
        )
        .fetch_all(pool)
        .await?;

        let api_keys = api_keys
            .into_iter()
            .filter_map(|user| match (user.api_key_id, user.owner_id) {
                (Some(api_key_id), Some(owner_id)) => Some(ApiKey {
                    api_key_id,
                    owner_id,
                    name: user.name,
                    secret: user.secret,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        let pagination = crate::Pagination {
            last_key: api_keys.last().map(|api_key| api_key.api_key_id),
            count: api_keys.len(),
        };

        Ok((api_keys, pagination))
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    api_key_id: Option<Ulid>,
    owner_id: Option<Ulid>,
    name: Option<String>,
    secret: Option<String>,
}

impl Builder {
    pub fn api_key_id(mut self, api_key_id: Ulid) -> Self {
        self.api_key_id = Some(api_key_id);
        self
    }

    pub fn owner_id(mut self, owner_id: Ulid) -> Self {
        self.owner_id = Some(owner_id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn secret(mut self, value: String) -> Self {
        self.secret = Some(value);
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = ApiKey;

    fn build(self) -> Result<Self::Item> {
        let api_key_id = self.api_key_id.or_else(|| Some(Ulid::generate())).unwrap();
        let owner_id = self
            .owner_id
            .ok_or_else(|| Error::ModelFieldsMissing("owner_id"))?;
        let name = self.name.ok_or_else(|| Error::ModelFieldsMissing("name"))?;
        let secret = self
            .secret
            .ok_or_else(|| Error::ModelFieldsMissing("secret"))?;

        Ok(ApiKey {
            api_key_id,
            owner_id,
            name,
            secret,
        })
    }
}
