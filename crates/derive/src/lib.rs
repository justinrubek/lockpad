use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UniqueEntity, attributes(unique_id))]
/// A UniqueEntity implements scylla_dynamodb::entity::GetKeys using T::PREFIX as the partition
/// key and ${T::PREFIX}#${T::ID} as the sort key.
/// The unique id will be marked by the attribute `#[unique_id]`.
pub fn unique_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(s) => {
            let name = input.ident;
            let fields = match s.fields {
                syn::Fields::Named(fields) => fields.named,
                _ => panic!("Only named fields are supported"),
            };

            let mut id_field = None;
            for field in fields {
                if field
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident("unique_id"))
                {
                    id_field = Some(field.ident.unwrap());
                }
            }

            let id_field = id_field.expect("No unique_id field found");

            let gen = quote! {
                impl scylla_dynamodb::entity::GetKeys for #name {
                    fn pk(
                        &self,
                        fields: &std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
                    ) -> aws_sdk_dynamodb::model::AttributeValue {
                        aws_sdk_dynamodb::model::AttributeValue::S(format!("{}", Self::PREFIX))
                    }

                    fn sk(
                        &self,
                        fields: &std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
                    ) -> aws_sdk_dynamodb::model::AttributeValue {
                        aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, self.#id_field))
                    }
                }

                impl scylla_dynamodb::entity::FormatKey for #name {
                    // the key is the unique id
                    type Key = String;

                    fn format_key(
                        key: Self::Key,
                    ) -> std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue> {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "pk".to_string(),
                            aws_sdk_dynamodb::model::AttributeValue::S(format!("{}", Self::PREFIX)),
                        );
                        map.insert(
                            "sk".to_string(),
                            aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, key)),
                        );
                        map
                    }
                }

                impl scylla_dynamodb::entity::QueryEntity for #name {
                    type Namespace = ();

                    fn query(
                        table: &scylla_dynamodb::DynamodbTable,
                        _key: Self::Namespace,
                    ) -> scylla_dynamodb::error::Result<aws_sdk_dynamodb::client::fluent_builders::Query> {
                        let pk_prefix = Self::PREFIX.to_string();
                        let pk_value = aws_sdk_dynamodb::model::AttributeValue::S(pk_prefix);
                        tracing::info!(?pk_value, "querying table");

                        let res = table
                            .client
                            .query()
                            .table_name(&table.name)
                            .key_condition_expression("#pk = :pk")
                            .expression_attribute_names("#pk", "pk")
                            .expression_attribute_values(":pk", pk_value);

                        Ok(res)
                    }
                }
            };

            gen.into()
        }
        _ => panic!("UniqueEntity can only be derived for structs"),
    }
}

#[proc_macro_derive(OwnedEntity, attributes(owner_id, object_id))]
/// A OwnedEntity implements scylla_dynamodb::entity::GetKeys using ${T::PREFIX}#${T::OWNER_ID} as the partition
/// key and ${T::PREFIX}#${T::OBJECT_ID} as the sort key.
/// The owner id will be marked by the attribute `#[owner_id]`.
/// The object id will be marked by the attribute `#[object_id]`.
pub fn owned_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(s) => {
            let name = input.ident;
            let fields = match s.fields {
                syn::Fields::Named(fields) => fields.named,
                _ => panic!("Only named fields are supported"),
            };

            let mut owner_id_field = None;
            let mut owner_id_type = None;
            let mut object_id_field = None;
            for field in fields {
                let ident = field.ident.unwrap();
                if field
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident("owner_id"))
                {
                    owner_id_field = Some(ident.clone());
                    owner_id_type = Some(field.ty.clone());
                }
                if field
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident("object_id"))
                {
                    object_id_field = Some(ident);
                }
            }

            let owner_id_field = owner_id_field.expect("No owner_id field found");
            let owner_id_type = owner_id_type.expect("No owner_id field found");
            let object_id_field = object_id_field.expect("No object_id field found");

            let gen = quote! {
                impl scylla_dynamodb::entity::GetKeys for #name {
                    fn pk(
                        &self,
                        fields: &std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
                    ) -> aws_sdk_dynamodb::model::AttributeValue {
                        aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, self.#owner_id_field))
                    }

                    fn sk(
                        &self,
                        fields: &std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
                    ) -> aws_sdk_dynamodb::model::AttributeValue {
                        aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, self.#object_id_field))
                    }
                }

                impl scylla_dynamodb::entity::FormatKey for #name {
                    type Key = (String, String);

                    fn format_key(
                        key: Self::Key,
                    ) -> std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue> {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            "pk".to_string(),
                            aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, key.0)),
                        );
                        map.insert(
                            "sk".to_string(),
                            aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, key.1)),
                        );
                        map
                    }
                }

                impl scylla_dynamodb::entity::QueryEntity for #name {
                    type Namespace = #owner_id_type;

                    fn query(
                        table: &scylla_dynamodb::DynamodbTable,
                        key: Self::Namespace,
                    ) -> scylla_dynamodb::error::Result<aws_sdk_dynamodb::client::fluent_builders::Query> {
                        let pk_prefix = Self::PREFIX.to_string();
                        let pk_value = aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", pk_prefix, key));
                        tracing::info!(?pk_value, "querying table");

                        let res = table
                            .client
                            .query()
                            .table_name(&table.name)
                            .key_condition_expression("#pk = :pk")
                            .expression_attribute_names("#pk", "pk")
                            .expression_attribute_values(":pk", pk_value);

                        Ok(res)
                    }
                }
            };

            gen.into()
        }
        _ => panic!("TenantEntity can only be derived for structs"),
    }
}
