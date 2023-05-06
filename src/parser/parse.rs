use chumsky::{
    prelude::Simple,
    primitive::just,
    recursive::{recursive, Recursive},
    text::{self, ident, TextParser},
    Parser,
};

use crate::codegen::{
    block::Block,
    item::Item,
    item_data::ItemData,
    misc::{BracketDirection, BracketType},
};

use super::datatypes::arguments_parser;

pub fn parser() -> impl Parser<char, Vec<Option<Block<'static>>>, Error = Simple<char>> {
    events_parser()
}

pub fn actions_parser() -> impl Parser<char, Vec<Option<Block<'static>>>, Error = Simple<char>> {
    let actions = recursive(
        |actions: Recursive<char, Vec<Option<Block>>, Simple<char>>| {
            // All lets have {} around them to allow them to be minified in code editors.
            // This allows for a easier time editing code!
            let operation = {
                just::<char, &str, Simple<char>>("=")
                    .or(just("+"))
                    .or(just("-"))
                    .or(just("*"))
                    .or(just("/"))
                    .or(just("%"))
            };

            let repeat = {
                text::keyword("loop")
                    .ignore_then(just(' '))
                    .ignore_then(ident())
                    .then_ignore(just("::"))
                    .then_ignore(ident())
                    .padded()
                    .then(argument_list())
                    .padded()
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
                    .map(|((label, args), codes)| {
                        let mut out = vec![];
                        for block in codes {
                            for sub_block in block.into_iter().flatten() {
                                out.append(&mut vec![Some(sub_block)]);
                            }
                        }
                        let mut items: Vec<Item> = vec![];
                        for (slot, data) in args.into_iter().enumerate() {
                            let id = data_to_id(&data);
                            items.push(Item {
                                id,
                                slot: slot.try_into().expect("failed ot convert to usize"),
                                item: data,
                            })
                        }
                        out.insert(
                            0,
                            Some(Block::Code {
                                block: "repeat",
                                items,
                                action: label,
                                data: "",
                                target: "",
                                inverted: "",
                                sub_action: String::new(),
                            }),
                        );
                        out.insert(
                            1,
                            Some(Block::Bracket {
                                direct: BracketDirection::Open,
                                typ: BracketType::Repeat,
                            }),
                        );
                        out.push(Some(Block::Bracket {
                            direct: BracketDirection::Close,
                            typ: BracketType::Repeat,
                        }));
                        out
                    })
            };

            let player_action = {
                text::keyword("player")
                    .ignore_then(just('.'))
                    .ignore_then(ident())
                    .then(argument_list())
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
                            target: "Selection",
                            inverted: "",
                            sub_action: String::new(),
                        })]
                    })
            };
            let game_action = {
                text::keyword("plot")
                    .ignore_then(just('.'))
                    .ignore_then(ident())
                    .then(argument_list())
                    .padded()
                    .map(|(f, datas): (String, Vec<ItemData>)| {
                        let mut items: Vec<Item> = vec![];
                        for (slot, data) in datas.into_iter().enumerate() {
                            let id = data_to_id(&data);

                            items.push(Item {
                                id,
                                slot: slot.try_into().expect("failed to convert to usize"),
                                item: data,
                            })
                        }
                        vec![Some(Block::Code {
                            block: "game_action",
                            items,
                            action: f,
                            data: "",
                            target: "Selection",
                            inverted: "",
                            sub_action: String::new(),
                        })]
                    })
            };

            let select_object = {
                text::keyword("select")
                    .padded()
                    .ignore_then(ident())
                    .then(argument_list())
                    .map(|(identifier, datas)| {
                        let mut items: Vec<Item> = vec![];
                        for (slot, data) in datas.into_iter().enumerate() {
                            let id = data_to_id(&data);

                            items.push(Item {
                                id,
                                slot: slot.try_into().expect("failed to convert to usize"),
                                item: data,
                            })
                        }
                        vec![Some(Block::Code {
                            block: "select_obj",
                            items,
                            action: identifier,
                            data: "",
                            target: "",
                            inverted: "",
                            sub_action: String::new(),
                        })]
                    })
            };

            let if_player = {
                text::keyword("if")
                    .ignore_then(just(' '))
                    .ignore_then(just('!'))
                    .ignore_then(text::keyword("player"))
                    .ignore_then(just('.'))
                    .ignore_then(ident())
                    .padded()
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
                            for sub_block in block.into_iter().flatten() {
                                out.append(&mut vec![Some(sub_block)]);
                            }
                        }
                        out.insert(
                            0,
                            Some(Block::Code {
                                block: "if_player",
                                items: vec![],
                                action: name,
                                data: "",
                                target: "Selection",
                                inverted: "",
                                sub_action: String::new(),
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
                    })
            };

            let if_entity = {
                text::keyword("if")
                    .ignore_then(just(' '))
                    .ignore_then(text::keyword("entity"))
                    .ignore_then(just('.'))
                    .ignore_then(ident())
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
                            for sub_block in block.into_iter().flatten() {
                                out.append(&mut vec![Some(sub_block)]);
                            }
                        }
                        out.insert(
                            0,
                            Some(Block::Code {
                                block: "if_entity",
                                items: vec![],
                                action: name,
                                data: "",
                                target: "Selection",
                                inverted: "",
                                sub_action: String::new(),
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
                    })
            };

            let if_game = {
                text::keyword("if")
                    .ignore_then(just(' '))
                    .ignore_then(text::keyword("plot"))
                    .ignore_then(just('.'))
                    .ignore_then(ident())
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
                            for sub_block in block.into_iter().flatten() {
                                out.append(&mut vec![Some(sub_block)]);
                            }
                        }
                        out.insert(
                            0,
                            Some(Block::Code {
                                block: "if_game",
                                items: vec![],
                                action: name,
                                data: "",
                                target: "Selection",
                                inverted: "",
                                sub_action: String::new(),
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
                    })
            };

            player_action
                .or(game_action)
                .or(repeat)
                .or(select_object)
                .or(if_player)
                .or(if_entity)
                .or(if_game)
        },
    );
    actions
}

pub fn events_parser() -> impl Parser<char, Vec<Option<Block<'static>>>, Error = Simple<char>> {
    let player_event = text::keyword("PlayerEvent")
        .ignore_then(just('('))
        .padded()
        .ignore_then(ident())
        .then_ignore(just(')'))
        .padded()
        .then(
            actions_parser()
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
                for sub_block in block.into_iter().flatten() {
                    out.append(&mut vec![Some(sub_block)]);
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

    let process = text::keyword("proc")
        .padded()
        .ignore_then(ident())
        .then_ignore(just('('))
        .padded()
        .then_ignore(just(')'))
        .padded()
        .then(
            actions_parser()
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
                for sub_block in block.into_iter().flatten() {
                    out.append(&mut vec![Some(sub_block)]);
                }
            }
            out.insert(
                0,
                Some(Block::ProcessDefinition {
                    block: "process",
                    data: name,
                }),
            );
            out
        });

    let function = text::keyword("func")
        .padded()
        .ignore_then(ident())
        .then_ignore(just('('))
        .padded()
        .then_ignore(just(')'))
        .padded()
        .then(
            actions_parser()
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
                for sub_block in block.into_iter().flatten() {
                    out.append(&mut vec![Some(sub_block)]);
                }
            }
            out.insert(
                0,
                Some(Block::FunctionDefinition {
                    block: "func",
                    data: name,
                }),
            );
            out
        });

    player_event.or(function).or(process)
}

fn data_to_id(data: &ItemData) -> String {
    match data {
        ItemData::Number { .. } => "num".to_string(),
        ItemData::Text { .. } => "txt".to_string(),
        ItemData::VanillaItem { .. } => "item".to_string(),
        ItemData::Location { .. } => "loc".to_string(),
        _ => "var".to_string(),
    }
}

pub fn argument_list() -> impl Parser<char, Vec<ItemData>, Error = Simple<char>> {
    arguments_parser()
        .separated_by(just(", "))
        .allow_trailing()
        .padded()
        .collect::<Vec<_>>()
        .padded()
        .delimited_by(just('('), just(')'))
        .padded()
}
