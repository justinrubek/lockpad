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
