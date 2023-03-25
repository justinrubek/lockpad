use crate::error::{Error, Result};
use lockpad_ulid::Ulid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub application_id: Ulid,
    pub owner_id: Ulid,

    pub name: String,
    pub allowed_origins: Vec<String>,
    pub allowed_callback_urls: Vec<String>,
}

impl Application {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn by_id(pool: &sqlx::pool::Pool<sqlx::Postgres>, id: &Ulid) -> Result<Option<Self>> {
        /*
        let application = sqlx::query_as!(
            Application,
            r#"
            SELECT
                application_id as "application_id: Ulid",
                owner_id as "owner_id: Ulid",
                name,
                allowed_origins,
                allowed_callback_urls
            FROM
                applications
            WHERE
                application_id = $1
            "#,
            id as _,
        )
        .fetch_one(pool)
        .await?;
        */

        let query = sqlx::query!(
            r#"
            SELECT
                application_id::uuid as "application_id: Ulid",
                owner_id::uuid as "owner_id: Ulid",
                name,
                allowed_origins,
                allowed_callback_urls
            FROM 
                applications
            WHERE 
                application_id::uuid = $1
            "#,
            id.to_sqlx_uuid()
        );

        let application = query.fetch_one(pool).await?;
        match (application.application_id, application.owner_id) {
            (Some(application_id), Some(owner_id)) => {
                let application = Application {
                    application_id,
                    owner_id,
                    name: application.name,
                    allowed_origins: application.allowed_origins,
                    allowed_callback_urls: application.allowed_callback_urls,
                };

                Ok(Some(application))
            }
            _ => Ok(None),
        }
    }

    pub async fn by_owner_id(
        pool: &sqlx::pool::Pool<sqlx::Postgres>,
        owner_id: &Ulid,
    ) -> Result<Option<Self>> {
        let application = sqlx::query_as!(
            Application,
            r#"
            SELECT
                application_id as "application_id: Ulid", owner_id as "owner_id: Ulid", name, allowed_origins, allowed_callback_urls
            FROM 
                applications
            WHERE 
                owner_id = $1
            "#,
            owner_id as _,
        )
        .fetch_one(pool)
        .await?;

        Ok(Some(application))
    }

    pub async fn create(&self, pool: &sqlx::pool::Pool<sqlx::Postgres>) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO 
                applications(application_id, owner_id, name, allowed_origins, allowed_callback_urls)
            SELECT 
                application_id::uuid, owner_id::uuid, name, allowed_origins, allowed_callback_urls
            FROM(
                VALUES(
                    $1, $2, $3, 
                    $4::text[], $5::text[]
                )
            ) AS data(application_id, owner_id, name, allowed_origins, allowed_callback_urls)
            "#,
        )
        .bind(self.application_id.queryable())
        .bind(self.owner_id.queryable())
        .bind(&self.name)
        .bind(&self.allowed_origins)
        .bind(&self.allowed_callback_urls)
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
        let applications = sqlx::query!(
            r#"
            SELECT
                application_id::uuid as "application_id: Ulid",
                owner_id::uuid as "owner_id: Ulid",
                name,
                allowed_origins,
                allowed_callback_urls
            FROM 
                applications
            "#,
        )
        .fetch_all(pool)
        .await?;

        let applications = applications
            .into_iter()
            .filter_map(|user| match (user.application_id, user.owner_id) {
                (Some(application_id), Some(owner_id)) => Some(Application {
                    application_id,
                    owner_id,
                    name: user.name,
                    allowed_origins: user.allowed_origins,
                    allowed_callback_urls: user.allowed_callback_urls,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        let pagination = crate::Pagination {
            last_key: applications
                .last()
                .map(|application| application.application_id),
            count: applications.len(),
        };

        Ok((applications, pagination))
    }
}

#[derive(Debug, Default)]
pub struct Builder {
    application_id: Option<Ulid>,
    owner_id: Option<Ulid>,
    name: Option<String>,
    allowed_origins: Option<Vec<String>>,
    allowed_callback_urls: Option<Vec<String>>,
}

impl Builder {
    pub fn application_id(mut self, application_id: Ulid) -> Self {
        self.application_id = Some(application_id);
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

    pub fn allowed_origins(mut self, allowed_origins: Vec<String>) -> Self {
        self.allowed_origins = Some(allowed_origins);
        self
    }

    pub fn allowed_callback_urls(mut self, allowed_callback_urls: Vec<String>) -> Self {
        self.allowed_callback_urls = Some(allowed_callback_urls);
        self
    }
}

impl crate::entity::Builder for Builder {
    type Item = Application;

    fn build(self) -> Result<Self::Item> {
        let application_id = self
            .application_id
            .or_else(|| Some(Ulid::generate()))
            .unwrap();
        let owner_id = self
            .owner_id
            .ok_or_else(|| Error::ModelFieldsMissing("owner_id"))?;
        let name = self.name.ok_or_else(|| Error::ModelFieldsMissing("name"))?;
        let allowed_origins = self
            .allowed_origins
            .ok_or_else(|| Error::ModelFieldsMissing("allowed_origins"))?;
        let allowed_callback_urls = self
            .allowed_callback_urls
            .ok_or_else(|| Error::ModelFieldsMissing("allowed_callback_urls"))?;

        Ok(Application {
            application_id,
            owner_id,
            name,
            allowed_origins,
            allowed_callback_urls,
        })
    }
}
