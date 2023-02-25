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
            let mut object_id_field = None;
            for field in fields {
                let ident = field.ident.unwrap();
                if field
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident("owner_id"))
                {
                    owner_id_field = Some(ident.clone());
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
            let object_id_field = object_id_field.expect("No object_id field found");

            let gen = quote! {
                impl scylla_dynamodb::entity::GetKeys for #name {
                    fn pk(
                        &self,
                        fields: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
                    ) -> Result<aws_sdk_dynamodb::model::AttributeValue> {
                        Ok(aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, self.#owner_id_field)))
                    }

                    fn sk(
                        &self,
                        fields: std::collections::HashMap<String, aws_sdk_dynamodb::model::AttributeValue>,
                    ) -> Result<aws_sdk_dynamodb::model::AttributeValue> {
                        Ok(aws_sdk_dynamodb::model::AttributeValue::S(format!("{}#{}", Self::PREFIX, self.#object_id_field)))
                    }
                }
            };

            gen.into()
        }
        _ => panic!("TenantEntity can only be derived for structs"),
    }
}
