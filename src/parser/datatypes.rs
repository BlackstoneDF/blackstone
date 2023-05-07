use super::ident;
use chumsky::extra::Err;
use chumsky::{
    prelude::Rich,
    primitive::{choice, just, none_of},
    text, IterParser, Parser,
};

use crate::codegen::item_data::ItemData;

use super::ident_to_var;

pub fn parse_number<'a>() -> impl Parser<'a, &'a str, ItemData, Err<Rich<'a, char>>> {
    // Number
    // This argument represents a Number type on DiamondFire.
    // It is parsed from an integer literal to a Number.
    let number = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .slice()
        .from_str()
        .unwrapped()
        .map(|f| ItemData::Number { data: f });
    number
}

pub fn parse_text<'a>() -> impl Parser<'a, &'a str, ItemData, Err<Rich<'a, char>>> {
    // Text
    // This argument represents a Text type on DiamondFire.
    // It is converted from a String literal.
    let text = just('"')
        .ignore_then(none_of('"').repeated().collect::<String>())
        .then_ignore(just('"'))
        .map(|f| ItemData::Text { data: f });

    text
}

pub fn parse_location<'a>() -> impl Parser<'a, &'a str, ItemData, Err<Rich<'a, char>>> {
    let location = text::keyword("loc")
        .ignore_then(parse_number().repeated().collect::<Vec<ItemData>>())
        .try_map(|f: Vec<ItemData>, f2| {
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
            return Err(Rich::custom(f2, "Locations must have 3 or 5 numbers."));
        });
    location
}

pub fn variable_parser<'a>() -> impl Parser<'a, &'a str, ItemData, Err<Rich<'a, char>>> {
    let variable = ident().map(|f: String| ident_to_var(f.as_str()));
    variable
}

pub fn parse_item_stack<'a>() -> impl Parser<'a, &'a str, ItemData, Err<Rich<'a, char>>> {
    let item = text::keyword("item")
        .ignore_then(parse_text().padded().delimited_by(just('('), just(')')))
        .try_map(|f, span| {
            if let ItemData::Text { data } = f {
                return Ok(ItemData::VanillaItem {
                    data: format!("{{Count:1b,DF_NBT:3337,id:\\\"minecraft:{data}\\\"}}"),
                });
            }
            return Err(Rich::custom(span, "Failed to provide valid item."));
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
                    return Err(Rich::custom(
                        f2,
                        "Invalid item stack. Requires an ID and an amount.",
                    ));
                };
            }
            unreachable!("Somehow reached an impossible Item Stack.");
        });

    item.or(item_stack).boxed()
}

pub fn arguments_parser<'a>() -> impl Parser<'a, &'a str, ItemData, Err<Rich<'a, char>>> {
    choice((
        parse_text(),
        parse_number(),
        parse_item_stack(),
        parse_location(),
        variable_parser(),
    ))
}
