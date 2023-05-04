use chumsky::{prelude::Simple, text, Parser, primitive::{just, none_of}};

use crate::codegen::item_data::ItemData;

pub fn parse_number() -> impl Parser<char, ItemData, Error = Simple<char>> {
    // Number
    // This argument represents a Number type on DiamondFire.
    // It is parsed from an integer literal to a Number.
    let number = text::int::<_, Simple<char>>(10).map(|f| ItemData::Number {
        data: f
            .parse::<f32>()
            .expect("failed to f32 somehow - shouldnt be possible"),
    });
    number
}

pub fn parse_text() -> impl Parser<char, ItemData, Error = Simple<char>> {
    // Text
    // This argument represents a Text type on DiamondFire.
    // It is converted from a String literal.
    let text = just::<char, char, Simple<char>>('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map(|f| ItemData::Text {
            data: f.iter().collect(),
        });

    text
}

pub fn arguments_parser() -> impl Parser<char, ItemData, Error = Simple<char>> {
    parse_number().or(parse_text())
}