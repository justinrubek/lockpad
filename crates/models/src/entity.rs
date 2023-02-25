use crate::error::Result;
use scylla_dynamodb::entity::PrefixedEntity;
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

// TODO: Determine response type for trait
// This could return a DynamoDb item hashmap
pub trait IdentityRef {
    fn id(&self) -> &PrimaryId;
}

impl IdentityRef for PrimaryId {
    fn id(&self) -> &PrimaryId {
        self
    }
}

pub trait EntityPrefix {
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
    fn put_item(
        &self,
        table: &scylla_dynamodb::DynamodbTable,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::PutItem>;
}

pub trait GetKeys {
    fn pk(
        &self,
        fields: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> Result<aws_sdk_dynamodb::model::AttributeValue>;

    fn sk(
        &self,
        fields: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> Result<aws_sdk_dynamodb::model::AttributeValue>;
}

impl<T: scylla_dynamodb::entity::GetKeys> PutEntity for T
where
    T: Entity + PrefixedEntity,
{
    fn put_item(
        &self,
        table: &scylla_dynamodb::DynamodbTable,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::PutItem> {
        let mut item = self.attributes()?;
        let pk = self.pk(&item);
        let sk = self.sk(&item);

        item.insert("pk".to_string(), pk);
        item.insert("sk".to_string(), sk);

        let res = table
            .client
            .put_item()
            .table_name(&table.name)
            .set_item(Some(item));

        Ok(res)
    }
}

pub trait Builder {
    type Item;

    fn build(self) -> Result<Self::Item>;
}
