use chumsky::{primitive::{one_of, end, none_of}, Parser, prelude::Simple};

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

pub fn ident() -> impl Parser<char, String, Error = Simple<char>> {
    let pt2 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789%<>";
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ%")
        .then(one_of(pt2).repeated())
        .then_ignore(none_of(pt2))
        .map(|(init_char, second_char)| {
            let collected = format!("{init_char}{}", second_char.into_iter().collect::<String>());
            let collected = collected.replace("<", "op").replace(">", "cl");
            collected
        })
}