use crate::codegen::{item_data::ItemData, misc::VariableScope};

pub mod datatypes;
pub mod parse;

fn ident_to_var(input: &str) -> ItemData {
    let mut words = input.split('.');
    if let Some(scope) = words.next() {
        match scope {
            "local" => ItemData::Variable {
                scope: VariableScope::Local,
                name: words.next().unwrap_or("_NULL").to_string(),
            },
            "save" => ItemData::Variable {
                scope: VariableScope::Saved,
                name: words.next().unwrap_or("_NULL").to_string(),
            },
            _ => ItemData::Variable {
                scope: VariableScope::Unsaved,
                name: words.next().unwrap_or("_NULL").to_string(),
            },
        }
    } else {
        ItemData::Variable {
            scope: VariableScope::Unsaved,
            name: words.next().unwrap_or("_NULL").to_string(),
        }
    }
}
