use chumsky::{
    prelude::Rich,
    primitive::{none_of, one_of},
    IterParser, Parser,
};

use crate::codegen::{item_data::ItemData, misc::VariableScope};
use chumsky::extra::Err;
pub mod datatypes;
pub mod parse;

pub fn ident<'a>() -> impl Parser<'a, &'a str, String, Err<Rich<'a, char>>> {
    let pt2 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789%<>";
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ%")
        .then(one_of(pt2).repeated().collect::<String>())
        .map(|(init_char, second_char)| {
            format!("{init_char}{second_char}").replace('<', "(").replace('>', ")")
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
