/// IR of a root field
use lang_graphql as gql;
use lang_graphql::ast::common as ast;
use open_dds::permissions::Role;
use serde::Serialize;

use super::{
    commands,
    query_root::{node_field, select_many, select_one},
};
use crate::schema::GDS;

/// IR of a root field
#[derive(Serialize, Debug)]
pub enum RootField<'n, 's> {
    QueryRootField(QueryRootField<'n, 's>),
    MutationRootField(MutationRootField<'n, 's>),
}

/// IR of a query root field
#[derive(Serialize, Debug)]
pub enum QueryRootField<'n, 's> {
    // __typename field on query root
    TypeName {
        type_name: ast::TypeName,
    },
    // __schema field
    SchemaField {
        role: Role,
        selection_set: &'n gql::normalized_ast::SelectionSet<'s, GDS>,
        schema: &'s gql::schema::Schema<GDS>,
    },
    // __type field
    TypeField {
        selection_set: &'n gql::normalized_ast::SelectionSet<'s, GDS>,
        schema: &'s gql::schema::Schema<GDS>,
        type_name: ast::TypeName,
        role: Role,
    },
    // Operation that selects a single row from a model
    ModelSelectOne {
        selection_set: &'n gql::normalized_ast::SelectionSet<'s, GDS>,
        ir: select_one::ModelSelectOne<'n, 's>,
    },
    // Operation that selects many rows from a model
    ModelSelectMany {
        selection_set: &'n gql::normalized_ast::SelectionSet<'s, GDS>,
        ir: select_many::ModelSelectMany<'n, 's>,
    },
    // Operation that selects a single row from the model corresponding
    // to the Global Id input.
    NodeSelect(Option<node_field::NodeSelect<'n, 's>>),
    CommandRepresentation {
        selection_set: &'n gql::normalized_ast::SelectionSet<'s, GDS>,
        ir: commands::CommandRepresentation<'n, 's>,
    },
}

/// IR of a mutation root field
#[derive(Serialize, Debug)]
pub enum MutationRootField<'n, 's> {
    // __typename field on mutation root
    TypeName {
        type_name: ast::TypeName,
    },
    CommandRepresentation {
        selection_set: &'n gql::normalized_ast::SelectionSet<'s, GDS>,
        ir: commands::CommandRepresentation<'n, 's>,
    },
}
