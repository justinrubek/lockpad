use crate::error::Result;
use serde::Serialize;

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
    fn query(
        &self,
        table: &crate::DynamodbTable,
    ) -> Result<aws_sdk_dynamodb::client::fluent_builders::Query>;
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

        let res = table
            .client
            .put_item()
            .table_name(&table.name)
            .set_item(Some(item));

        Ok(res)
    }
}
