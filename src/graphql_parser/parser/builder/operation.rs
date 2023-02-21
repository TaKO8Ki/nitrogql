use crate::{
    graphql_parser::ast::{
        base::Pos,
        operations::{VariableDefinition, VariablesDefinition},
    },
    parts,
};

use super::{
    base::build_variable, directives::build_directives, r#type::build_type, utils::PairExt,
    value::build_value, Rule,
};
use pest::iterators::Pair;

/// Parses a VariablesDefinition Pair.
pub fn build_variables_definition(pair: Pair<Rule>) -> VariablesDefinition {
    let position: Pos = (&pair).into();
    let defs = pair.all_children(Rule::VariableDefinition);
    VariablesDefinition {
        position,
        definitions: defs.into_iter().map(build_variable_definition).collect(),
    }
}

/// Parses one VariableDefinition Pair.
pub fn build_variable_definition(pair: Pair<Rule>) -> VariableDefinition {
    let pos: Pos = (&pair).into();
    let (variable, ty, default_value, directives) = parts!(
        pair.into_inner(),
        Variable,
        Type,
        DefaultValue opt,
        Directives opt
    );

    VariableDefinition {
        pos,
        name: build_variable(variable),
        r#type: build_type(ty),
        default_value: default_value.map(|pair| {
            let child = pair.only_child();
            build_value(child)
        }),
        directives: directives.map_or(vec![], build_directives),
    }
}