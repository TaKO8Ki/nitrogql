use std::collections::HashMap;

use nitrogql_ast::operation::{ExecutableDefinition, FragmentDefinition, OperationDocument};

pub type FragmentMap<'a, 'src> = HashMap<&'a str, &'a FragmentDefinition<'src>>;

pub fn generate_fragment_map<'a, 'src>(
    document: &'a OperationDocument<'src>,
) -> FragmentMap<'a, 'src> {
    document
        .definitions
        .iter()
        .flat_map(|def| {
            if let ExecutableDefinition::FragmentDefinition(fr) = def {
                Some((fr.name.name, fr))
            } else {
                None
            }
        })
        .collect()
}
