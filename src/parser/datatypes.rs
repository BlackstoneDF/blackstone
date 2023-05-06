use chumsky::{
    prelude::Simple,
    primitive::{just, none_of},
    text::{self, TextParser},
    Parser,
};
use super::ident;

use crate::codegen::item_data::ItemData;

use super::ident_to_var;

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

pub fn parse_location() -> impl Parser<char, ItemData, Error = Simple<char>> {
    let location = text::keyword("loc")
        .ignore_then(
            parse_number()
                .separated_by(just(','))
                .allow_trailing()
                .padded()
                .collect::<Vec<_>>()
                .padded()
                .delimited_by(just('('), just(')')),
        )
        .try_map(|f, f2| {
            if f.len() == 3 {
                if let Some(ItemData::Number { data: n1 }) = f.get(0) {
                    if let Some(ItemData::Number { data: n2 }) = f.get(1) {
                        if let Some(ItemData::Number { data: n3 }) = f.get(2) {
                            return Ok(ItemData::Location {
                                x: *n1,
                                y: *n2,
                                z: *n3,
                                pitch: 0.0,
                                yaw: 0.0,
                            });
                        }
                    }
                }
            } else if f.len() == 5 {
                if let Some(ItemData::Number { data: n1 }) = f.get(0) {
                    if let Some(ItemData::Number { data: n2 }) = f.get(1) {
                        if let Some(ItemData::Number { data: n3 }) = f.get(2) {
                            if let Some(ItemData::Number { data: n4 }) = f.get(3) {
                                if let Some(ItemData::Number { data: n5 }) = f.get(4) {
                                    return Ok(ItemData::Location {
                                        x: *n1,
                                        y: *n2,
                                        z: *n3,
                                        pitch: *n4,
                                        yaw: *n5,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // Report::build(ReportKind::Warning, (), 5);
            // TODO: throw ariadne error
            Err(Simple::custom(f2, "Locations need 3 or 5 fields."))
        });
    location
}

pub fn variable_parser() -> impl Parser<char, ItemData, Error = Simple<char>> {
    let variable = ident().map(|f: String| ident_to_var(f.as_str()));
    variable
}

pub fn parse_item_stack() -> impl Parser<char, ItemData, Error = Simple<char>> {
    let item = text::keyword("item")
        .ignore_then(parse_text().padded().delimited_by(just('('), just(')')))
        .map(|f| {
            if let ItemData::Text { data } = f {
                return ItemData::VanillaItem {
                    data: format!("{{Count:1b,DF_NBT:3337,id:\\\"minecraft:{data}\\\"}}"),
                };
            }
            ItemData::VanillaItem {
                data: "".to_string(),
            }
        });

    let item_stack = text::keyword("items")
        .ignore_then(parse_text().padded().delimited_by(just('('), just(')')))
        .try_map(|f, f2| {
            if let ItemData::Text { data } = f {
                let split = data.split(':').collect::<Vec<_>>();
                let id = split.first().expect("somehow failed");
                if let Some(count) = split.get(1) {
                    return Ok(ItemData::VanillaItem {
                        data: format!("{{Count:{count}b,DF_NBT:3337,id:\\\"minecraft:{id}\\\"}}"),
                    });
                } else {
                    return Err(Simple::custom(f2, "Item Stacks must have an ID followed by a count. Example: `coal:3` makes 3 coals in a singular stack."))
                };
            }
            unreachable!();
        });

    item.or(item_stack)
}

pub fn arguments_parser() -> impl Parser<char, ItemData, Error = Simple<char>> {
    parse_number()
        .or(parse_text())
        .or(parse_location())
        .or(parse_item_stack())
        .or(variable_parser())
}
