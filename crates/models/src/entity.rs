use crate::error::{Error, Result};
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

/// A UniqueEntity is something with a unique field.
/// This field will be encoded in the object's key to ensure uniqueness.
pub trait UniqueEntity {
    /// Retrieve the unique value for this entity.
    fn unique_field(
        &self,
        fields: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
    ) -> Result<Option<aws_sdk_dynamodb::model::AttributeValue>>;
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

impl<T> PutEntity for T
where
    T: Entity + UniqueEntity + EntityPrefix + Send + std::marker::Sync,
{
    fn put_item(
        &self,
        table: &scylla_dynamodb::DynamodbTable,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::PutItem> {
        // TODO: Implement PutItem for unique entitiy. Base this on the user model.
        let mut item = self.attributes()?;
        let unique_field = self.unique_field(item.clone())?.unwrap();

        // Add the prefix to the unique field (`{prefix}#{unique_field}`)
        let unique_attr = match unique_field {
            AttributeValue::S(s) => AttributeValue::S(format!("{}#{}", T::PREFIX, s)),
            AttributeValue::N(n) => AttributeValue::S(format!("{}#{}", T::PREFIX, n)),
            _ => return Err(Error::InvalidUniqueField),
        };

        item.insert("pk".to_string(), AttributeValue::S(T::PREFIX.to_string()));
        item.insert("sk".to_string(), unique_attr);

        let res = table
            .client
            .put_item()
            .table_name(&table.name)
            .set_item(Some(item));

        Ok(res)
    }
}
