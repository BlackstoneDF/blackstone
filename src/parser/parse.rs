use std::sync::{Arc, Mutex};

use crate::codegen::misc::{BracketDirection, BracketType};
use crate::codegen::{block::Block, item::Item, item_data::ItemData};
use crate::parser::actions::*;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Vec<Option<Block<'static>>>, Error = Simple<char>> {
    let player_default: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    let ident = text::ident();

    // Type Command
    // This command represents creating a type that references a selector.
    // You can use these to call different actions.
    let type_command = text::keyword("type")
        .ignore_then(just(' '))
        .ignore_then(ident)
        .padded()
        .then_ignore(just('='))
        .padded()
        .then_ignore(ident)
        .padded()
        .ignore_then(just(""))
        .map(move |varn: &str| {
            let cl = Arc::clone(&player_default);
            cl.lock()
                .expect("poisoned lock, immediately panic")
                .push(varn.to_string());
            vec![None::<Block>]
        });

    let internal_commands = { type_command };

    // Number
    // This argument represents a Number type on DiamondFire.
    // It is parsed from an integer literal to a Number.
    let number = text::int::<_, Simple<char>>(10).map(|f| ItemData::Number {
        data: f
            .parse::<f32>()
            .expect("failed to f32 somehow - shouldnt be possible"),
    });

    // Text
    // This argument represents a Text type on DiamondFire.
    // It is converted from a String literal.
    let text = just::<char, char, Simple<char>>('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map(|f| ItemData::Text {
            data: f.iter().collect(),
        });

    // Location
    // This argument represents a Location type on diamondfire.
    // It is parsed from 3 or 5 arguments inside angle brackets.
    // loc(1, 2, 3) VALID, loc(1, 2, 3, 4, 5) VALID, loc(1, 2, 3, 4) INVALID, loc(1, 2) INVALID
    let location = text::keyword("loc")
        .ignore_then(
            number
                .clone()
                .separated_by(just(','))
                .allow_trailing()
                .padded()
                .collect::<Vec<_>>()
                .padded()
                .delimited_by(just('('), just(')')),
        )
        .map(|f| {
            if f.len() == 3 {
                if let Some(ItemData::Number { data: n1 }) = f.get(0) {
                    if let Some(ItemData::Number { data: n2 }) = f.get(1) {
                        if let Some(ItemData::Number { data: n3 }) = f.get(2) {
                            return ItemData::Location {
                                x: *n1,
                                y: *n2,
                                z: *n3,
                                pitch: 0.0,
                                yaw: 0.0,
                            };
                        }
                    }
                }
                // TODO: throw ariadne error
                return ItemData::Location {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    pitch: 0.0,
                    yaw: 0.0,
                };
            } else if f.len() == 5 {
                if let Some(ItemData::Number { data: n1 }) = f.get(0) {
                    if let Some(ItemData::Number { data: n2 }) = f.get(1) {
                        if let Some(ItemData::Number { data: n3 }) = f.get(2) {
                            if let Some(ItemData::Number { data: n4 }) = f.get(3) {
                                if let Some(ItemData::Number { data: n5 }) = f.get(4) {
                                    return ItemData::Location {
                                        x: *n1,
                                        y: *n2,
                                        z: *n3,
                                        pitch: *n4,
                                        yaw: *n5,
                                    };
                                }
                            }
                        }
                    }
                }
                // TODO: throw ariadne error
                return ItemData::Location {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    pitch: 0.0,
                    yaw: 0.0,
                };
            }
            // TODO: throw ariadne error
            return ItemData::Location {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                pitch: 0.0,
                yaw: 0.0,
            };
        });

    let arguments = text.or(number).or(location);

    let actions = recursive(|actions| {
        let player_action = text::keyword("player")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then(
                arguments
                    .clone()
                    .separated_by(just(", "))
                    .allow_trailing()
                    .padded()
                    .collect::<Vec<_>>()
                    .padded()
                    .delimited_by(just('('), just(')'))
                    .padded(),
            )
            .padded()
            .map(|(f, datas): (String, Vec<ItemData>)| {
                let mut items: Vec<Item> = vec![];
                for (slot, data) in datas.into_iter().enumerate() {
                    let id = data_to_id(&data);

                    items.push(Item {
                        id,
                        slot: slot.try_into().expect("failed ot convert to usize"),
                        item: data,
                    })
                }
                vec![Some(Block::Code {
                    block: "player_action",
                    items,
                    action: f,
                    data: "",
                    target: "Default",
                    inverted: "",
                })]
            });

        let game_action = text::keyword("plot")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then(
                arguments
                    .clone()
                    .separated_by(just(',').padded())
                    .allow_trailing()
                    .padded()
                    .collect::<Vec<_>>()
                    .padded()
                    .delimited_by(just('('), just(')'))
                    .padded(),
            )
            .padded()
            .map(|(f, datas): (String, Vec<ItemData>)| {
                let mut items: Vec<Item> = vec![];
                for (slot, data) in datas.into_iter().enumerate() {
                    let id = data_to_id(&data);

                    items.push(Item {
                        id,
                        slot: slot.try_into().expect("failed ot convert to usize"),
                        item: data,
                    })
                }
                vec![Some(Block::Code {
                    block: "game_action",
                    items,
                    action: f,
                    data: "",
                    target: "Default",
                    inverted: "",
                })]
            });


        let if_player = text::keyword("if")
            .ignore_then(just(' '))
            .ignore_then(text::keyword("player"))
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then(
                actions
                    .clone()
                    .separated_by(just(';'))
                    .allow_trailing()
                    .padded()
                    .collect::<Vec<_>>()
                    .padded()
                    .delimited_by(just('{'), just('}'))
                    .padded(),
            )
            .padded()
            .map(|(name, args): (String, Vec<Vec<Option<Block>>>)| {
                let mut out = vec![];
                for block in args {
                    for sub_block in block {
                        if let Some(bl) = sub_block {
                            out.append(&mut vec![Some(bl)]);
                        }
                    }
                }
                out.insert(
                    0,
                    Some(Block::Code {
                        block: "if_player",
                        items: vec![],
                        action: name,
                        data: "",
                        target: "Default",
                        inverted: "",
                    }),
                );
                out.insert(
                    1,
                    Some(Block::Bracket {
                        direct: BracketDirection::Open,
                        typ: BracketType::Norm,
                    }),
                );
                out.push(Some(Block::Bracket {
                    direct: BracketDirection::Close,
                    typ: BracketType::Norm,
                }));
                out
            });

        player_action.or(game_action).or(if_player)
    });

    let player_event = text::keyword("PlayerEvent")
        .ignore_then(just('('))
        .ignore_then(text::keyword("event"))
        .ignore_then(just(':'))
        .padded()
        .ignore_then(ident)
        .then_ignore(just(')'))
        .padded()
        .then(
            actions
                .separated_by(just(';'))
                .allow_trailing()
                .padded()
                .collect::<Vec<_>>()
                .padded()
                .delimited_by(just('{'), just('}'))
                .padded(),
        )
        .padded()
        .map(|(name, args): (String, Vec<Vec<Option<Block>>>)| {
            let mut out = vec![];
            for block in args {
                for sub_block in block {
                    if let Some(bl) = sub_block {
                        out.append(&mut vec![Some(bl)]);
                    }
                }
            }
            out.insert(
                0,
                Some(Block::EventDefinition {
                    block: "event",
                    action: name,
                }),
            );
            out
        });

    let events = { player_event };

    events
}

fn data_to_id(data: &ItemData) -> String {
    if let ItemData::Number { .. } = data {
        return "num".to_string();
    }
    if let ItemData::Text { .. } = data {
        return "txt".to_string();
    }
    if let ItemData::Location { .. } = data {
        return "loc".to_string();
    }
    "".to_string()
}
