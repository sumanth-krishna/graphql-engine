/*
This module provides functions to generate introspection result as GraphQL schema
for each namespace from the schema.
 */
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to parse introspection query: {0}")]
    ParseIntrospectionQuery(String),
    #[error("unable to normalize introspection query: {0}")]
    NormalizeIntrospectionQuery(String),
    #[error("unable to find field call")]
    FieldCallNotFound,
    #[error("Only __schema field is expected but found: {name:}")]
    OnlySchemaFieldExpected { name: String },
    #[error("introspection query failed: {0}")]
    IntrospactionQueryError(#[from] crate::introspection::Error),
    #[error("unable to serialize to json: {0}")]
    SerializeJson(#[from] serde_json::Error),
}

lazy_static::lazy_static! {

    static ref INTROSPECTION_REQUEST: Result<crate::http::Request, String>  = {
        Ok(
            crate::http::Request {
                operation_name: None,
                query: {
                    let query_str = include_str!("introspection_query.graphql");
                    crate::parser::Parser::new(query_str)
                        .parse_executable_document()
                        .map_err(|e| e.to_string())?
                },
                variables: HashMap::new(),
            }
        )
    };
}

/// Generate GraphQL schema for a given namespace
pub fn build_namespace_schema<
    S: crate::schema::SchemaContext,
    NSGet: crate::schema::NamespacedGetter<S>,
>(
    namespaced_getter: &NSGet,
    schema: &crate::schema::Schema<S>,
) -> Result<serde_json::Value, Error> {
    let request = match &(*INTROSPECTION_REQUEST) {
        Ok(req) => req,
        Err(e) => Err(Error::ParseIntrospectionQuery((*e).clone()))?,
    };
    let nr = crate::validation::normalize_request(namespaced_getter, schema, request)
        .map_err(|e| Error::NormalizeIntrospectionQuery(e.to_string()))?;
    let mut result = HashMap::new();
    for (_alias, field) in &nr.selection_set.fields {
        let field_call = field.field_call().map_err(|_| Error::FieldCallNotFound)?;
        match field_call.name.as_str() {
            "__schema" => {
                result.insert(
                    &field_call.name,
                    serde_json::to_value(crate::introspection::schema_type(
                        schema,
                        namespaced_getter,
                        &field.selection_set,
                    )?)?,
                );
            }
            name => Err(Error::OnlySchemaFieldExpected {
                name: name.to_string(),
            })?,
        }
    }
    Ok(serde_json::to_value(result)?)
}
