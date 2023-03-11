use crate::error::Result;
use aws_sdk_dynamodb::model::AttributeValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrimaryId(ulid::Ulid);

impl Default for PrimaryId {
    fn default() -> Self {
        Self(ulid::Ulid::new())
    }
}

impl PrimaryId {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::str::FromStr for PrimaryId {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(ulid::Ulid::from_str(s)?))
    }
}

impl std::fmt::Display for PrimaryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait GetKeys {
    fn pk(
        &self,
        fields: &std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> aws_sdk_dynamodb::model::AttributeValue;

    fn sk(
        &self,
        fields: &std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> aws_sdk_dynamodb::model::AttributeValue;
}

pub trait FormatKey {
    type Key;

    fn format_key(key: Self::Key) -> std::collections::HashMap<String, AttributeValue>;
}

pub trait PrefixedEntity {
    const PREFIX: &'static str;
}

pub trait Entity {
    fn attributes(
        &self,
    ) -> Result<std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>>;
}

impl<T> Entity for T
where
    T: Serialize,
{
    fn attributes(
        &self,
    ) -> Result<std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>> {
        Ok(serde_dynamo::to_item(self)?)
    }
}

pub trait PutEntity {
    type Error;

    fn put_item(
        &self,
        table: &crate::DynamodbTable,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::PutItem>;
}

pub trait QueryEntity {
    type Namespace;

    fn query(
        table: &crate::DynamodbTable,
        namespace: Self::Namespace,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::Query>;
}

pub trait GetEntity {
    fn get(
        table: &crate::DynamodbTable,
        key: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::GetItem>;
}

impl<T: GetKeys> PutEntity for T
where
    T: Entity + PrefixedEntity,
{
    type Error = crate::error::Error;

    fn put_item(
        &self,
        table: &crate::DynamodbTable,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::PutItem> {
        let mut item = self.attributes()?;
        let pk = self.pk(&item);
        let sk = self.sk(&item);

        item.insert("pk".to_string(), pk);
        item.insert("sk".to_string(), sk);

        tracing::info!(?item, "putting item");

        let res = table
            .client
            .put_item()
            .table_name(&table.name)
            .set_item(Some(item));

        Ok(res)
    }
}

impl<T: FormatKey> GetEntity for T
where
    T: Entity,
{
    fn get(
        table: &crate::DynamodbTable,
        key: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::GetItem> {
        let res = table
            .client
            .get_item()
            .table_name(&table.name)
            .set_key(Some(key));

        Ok(res)
    }
}
