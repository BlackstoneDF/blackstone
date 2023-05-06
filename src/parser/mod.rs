use chumsky::{Parser, primitive::{one_of, none_of}, IterParser, regex::regex, prelude::Rich, extra};

use crate::codegen::{item_data::ItemData, misc::VariableScope};
use chumsky::extra::Err;
pub mod datatypes;
pub mod parse;

pub fn ident<'a>() -> impl Parser<'a, &'a str, String, Err<Rich<'a, char>>> {
    let pt2 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789%<>";
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ%")
        .then(one_of(pt2).repeated().collect::<String>())
        .then_ignore(none_of(pt2))
        .map(|(init_char, second_char)| {
            let collected = format!("{init_char}{second_char}");
            let collected = collected.replace("<", "(").replace(">", ")");
            collected
        })
}

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
