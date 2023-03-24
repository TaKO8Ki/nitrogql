use graphql_type_system::{
    DirectiveDefinition, EnumDefinition, EnumMember, Field, InputObjectDefinition, InputValue,
    InterfaceDefinition, Node, ObjectDefinition, ScalarDefinition, Schema, SchemaBuilder,
    TypeDefinition, UnionDefinition,
};
use nitrogql_ast::{
    base::{HasPos, Pos},
    operation::OperationType,
    type_system::{
        ArgumentsDefinition, FieldDefinition, TypeDefinition as AstTypeDefinition,
        TypeSystemDefinition,
    },
    value::StringValue,
    TypeSystemDocument,
};

use crate::type_system_utils::{convert_type, ident_to_node};

/// Convert TypeSystemDocument AST to type system struct.
pub fn ast_to_type_system<'a>(document: &'a TypeSystemDocument) -> Schema<&'a str, Pos> {
    let mut builder = SchemaBuilder::<&'a str, Pos>::new();
    for def in document.definitions.iter() {
        match def {
            TypeSystemDefinition::SchemaDefinition(ref def) => {
                if let Some(ref desc) = def.description {
                    builder.set_description(Node::from(&desc.value, desc.position));
                }
                let root_types = builder.set_root_types(def.position);
                for (operation, def) in def.definitions.iter() {
                    match operation {
                        OperationType::Query => root_types.set_query_type(ident_to_node(def)),
                        OperationType::Mutation => root_types.set_mutation_type(ident_to_node(def)),
                        OperationType::Subscription => {
                            root_types.set_subscription_type(ident_to_node(def))
                        }
                    }
                }
            }
            TypeSystemDefinition::TypeDefinition(ref def) => match def {
                AstTypeDefinition::Scalar(ref def) => builder.extend(vec![(
                    def.name.name,
                    Node::from(
                        TypeDefinition::Scalar(ScalarDefinition {
                            name: ident_to_node(&def.name),
                            description: convert_description(&def.description),
                        }),
                        def.position,
                    ),
                )]),
                AstTypeDefinition::Object(ref def) => builder.extend(vec![(
                    def.name.name,
                    Node::from(
                        TypeDefinition::Object(ObjectDefinition {
                            name: ident_to_node(&def.name),
                            description: convert_description(&def.description),
                            fields: def.fields.iter().map(convert_field).collect(),
                            interfaces: def.implements.iter().map(ident_to_node).collect(),
                        }),
                        def.position,
                    ),
                )]),
                AstTypeDefinition::Interface(ref def) => builder.extend(vec![(
                    def.name.name,
                    Node::from(
                        TypeDefinition::Interface(InterfaceDefinition {
                            name: ident_to_node(&def.name),
                            description: convert_description(&def.description),
                            fields: def.fields.iter().map(convert_field).collect(),
                            interfaces: def.implements.iter().map(ident_to_node).collect(),
                        }),
                        def.position,
                    ),
                )]),
                AstTypeDefinition::Union(ref def) => builder.extend(vec![(
                    def.name.name,
                    Node::from(
                        TypeDefinition::Union(UnionDefinition {
                            name: ident_to_node(&def.name),
                            description: convert_description(&def.description),
                            possible_types: def.members.iter().map(ident_to_node).collect(),
                        }),
                        def.position,
                    ),
                )]),
                AstTypeDefinition::Enum(ref def) => builder.extend(vec![(
                    def.name.name,
                    Node::from(
                        TypeDefinition::Enum(EnumDefinition {
                            name: ident_to_node(&def.name),
                            description: convert_description(&def.description),
                            members: def
                                .values
                                .iter()
                                .map(|mem| EnumMember {
                                    name: ident_to_node(&mem.name),
                                })
                                .collect(),
                        }),
                        def.position,
                    ),
                )]),
                AstTypeDefinition::InputObject(ref def) => builder.extend(vec![(
                    def.name.name,
                    Node::from(
                        TypeDefinition::InputObject(InputObjectDefinition {
                            name: ident_to_node(&def.name),
                            description: convert_description(&def.description),
                            fields: def
                                .fields
                                .iter()
                                .map(|input| InputValue {
                                    name: ident_to_node(&input.name),
                                    description: convert_description(&input.description),
                                    r#type: convert_type(&input.r#type),
                                    default_value: input
                                        .default_value
                                        .as_ref()
                                        .map(|value| Node::from((), *value.position())),
                                })
                                .collect(),
                        }),
                        def.position,
                    ),
                )]),
            },
            TypeSystemDefinition::DirectiveDefinition(ref def) => builder.extend(vec![(
                def.name.name,
                DirectiveDefinition {
                    name: ident_to_node(&def.name),
                    description: convert_description(&def.description),
                    locations: def
                        .locations
                        .iter()
                        .map(|loc| Node::from(loc.name, loc.position))
                        .collect(),
                    arguments: convert_arguments(&def.arguments),
                    repeatable: def.repeatable.map(|ident| Node::from((), ident.position)),
                },
            )]),
        }
    }

    builder.into()
}

fn convert_description<'src>(description: &Option<StringValue>) -> Option<Node<&str, Pos>> {
    description
        .as_ref()
        .map(|desc| Node::from(desc.value.as_ref(), desc.position))
}

fn convert_arguments<'src, 'a: 'src>(
    arguments: &'a Option<ArgumentsDefinition<'src>>,
) -> Vec<InputValue<&'src str, Pos>> {
    arguments.as_ref().map_or(vec![], |args| {
        args.input_values
            .iter()
            .map(|input| InputValue {
                name: ident_to_node(&input.name),
                description: convert_description(&input.description),
                r#type: convert_type(&input.r#type),
                default_value: input
                    .default_value
                    .as_ref()
                    .map(|value| Node::from((), *value.position())),
            })
            .collect()
    })
}

fn convert_field<'src, 'a: 'src>(field: &'a FieldDefinition<'src>) -> Field<&'src str, Pos> {
    Field {
        name: ident_to_node(&field.name),
        description: convert_description(&field.description),
        r#type: convert_type(&field.r#type),
        arguments: convert_arguments(&field.arguments),
    }
}
