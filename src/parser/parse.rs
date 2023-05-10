use chumsky::{
    prelude::Rich,
    primitive::{choice, just},
    recursive::recursive,
    text, IterParser, Parser,
};

use crate::codegen::{
    block::Block,
    item::Item,
    item_data::ItemData,
    misc::{BracketDirection, BracketType},
};
use chumsky::extra::Err;

use super::{
    datatypes::{arguments_parser, variable_parser},
    ident,
};

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Vec<Option<Block<'a>>>>, Err<Rich<'a, char>>> {
    events_parser().repeated().collect::<Vec<_>>()
}

pub fn actions_parser<'a>() -> impl Parser<'a, &'a str, Vec<Option<Block<'a>>>, Err<Rich<'a, char>>>
{
    let operation = {
        just::<&str, &str, Err<Rich<'a, char>>>("=")
            .or(just("+"))
            .or(just("-"))
            .or(just("*"))
            .or(just("/"))
            .or(just("%"))
    };

    let recurs = recursive(|actions| {
        /*
        ACTIONS
         */
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
                        action: first_upper(&f),
                        data: "",
                        target: "Selection",
                        inverted: "",
                        sub_action: String::new(),
                    })]
                })
        }
        .boxed();

        let entity_action = {
            text::keyword("entity")
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
                        block: "entity_action",
                        items,
                        action: first_upper(&f),
                        data: "",
                        target: "Selection",
                        inverted: "",
                        sub_action: String::new(),
                    })]
                })
        }
        .boxed();

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
                        action: first_upper(&f),
                        data: "",
                        target: "Selection",
                        inverted: "",
                        sub_action: String::new(),
                    })]
                })
        }
        .boxed();

        /*
        VARIABLES
         */

        let set_variable_local = {
            text::keyword("local")
                .padded()
                .ignore_then(variable_parser())
                .padded()
                .then(operation)
                .padded()
                .then(ident())
                .padded()
                .then(argument_list())
                .map(
                    |(((var, op), effect), args): (((ItemData, &str), String), Vec<ItemData>)| {
                        let mut items: Vec<Item> = vec![];
                        for (slot, data) in args.into_iter().enumerate() {
                            let id = data_to_id(&data);
                            let slot = slot + 1;
                            items.push(Item {
                                id,
                                slot: slot.try_into().expect("failed ot convert to usize"),
                                item: data,
                            })
                        }
                        items.insert(
                            0,
                            Item {
                                slot: 0,
                                id: "var".to_string(),
                                item: var,
                            },
                        );
                        let mut tmp_effect = effect;
                        if tmp_effect == "with" {
                            tmp_effect = op.to_string();
                        }
                        vec![Some(Block::Code {
                            block: "set_var",
                            items,
                            action: first_upper(&tmp_effect),
                            data: "",
                            target: "",
                            inverted: "",
                            sub_action: String::new(),
                        })]
                    },
                )
        }
        .boxed();

        let set_variable_game = {
            text::keyword("game")
                .padded()
                .ignore_then(variable_parser())
                .padded()
                .then(operation)
                .padded()
                .then(ident())
                .padded()
                .then(argument_list())
                .map(
                    |(((var, op), effect), args): (((ItemData, &str), String), Vec<ItemData>)| {
                        let mut items: Vec<Item> = vec![];
                        for (slot, data) in args.into_iter().enumerate() {
                            let id = data_to_id(&data);
                            let slot = slot + 1;
                            items.push(Item {
                                id,
                                slot: slot.try_into().expect("failed ot convert to usize"),
                                item: data,
                            })
                        }
                        items.insert(
                            0,
                            Item {
                                slot: 0,
                                id: "var".to_string(),
                                item: var,
                            },
                        );
                        let mut tmp_effect = effect;
                        if tmp_effect == "with" {
                            tmp_effect = op.to_string();
                        }
                        vec![Some(Block::Code {
                            block: "set_var",
                            items,
                            action: first_upper(&tmp_effect),
                            data: "",
                            target: "",
                            inverted: "",
                            sub_action: String::new(),
                        })]
                    },
                )
        }
        .boxed();

        let set_variable_saved = {
            text::keyword("saved")
                .padded()
                .ignore_then(variable_parser())
                .padded()
                .then(operation)
                .padded()
                .then(ident())
                .padded()
                .then(argument_list())
                .map(
                    |(((var, op), effect), args): (((ItemData, &str), String), Vec<ItemData>)| {
                        let mut items: Vec<Item> = vec![];
                        for (slot, data) in args.into_iter().enumerate() {
                            let id = data_to_id(&data);
                            let slot = slot + 1;
                            items.push(Item {
                                id,
                                slot: slot.try_into().expect("failed ot convert to usize"),
                                item: data,
                            })
                        }
                        items.insert(
                            0,
                            Item {
                                slot: 0,
                                id: "var".to_string(),
                                item: var,
                            },
                        );
                        let mut tmp_effect = effect;
                        if tmp_effect == "with" {
                            tmp_effect = op.to_string();
                        }
                        vec![Some(Block::Code {
                            block: "set_var",
                            items,
                            action: first_upper(&tmp_effect),
                            data: "",
                            target: "",
                            inverted: "",
                            sub_action: String::new(),
                        })]
                    },
                )
        }
        .boxed();

        /*
        IFS
         */

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
                            action: first_upper(&name),
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
        }
        .boxed();

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
                            action: first_upper(&name),
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
        }
        .boxed();

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
                            action: first_upper(&name),
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
        }
        .boxed();

        /*
        OTHER
         */
        choice((
            player_action,
            entity_action,
            game_action,
            if_player,
            if_entity,
            if_game,
            set_variable_game,
            set_variable_local,
            set_variable_saved,
        ))
    });

    return recurs;
}

pub fn events_parser<'a>() -> impl Parser<'a, &'a str, Vec<Option<Block<'a>>>, Err<Rich<'a, char>>>
{
    let player_event = text::keyword("event")
        .padded()
        .ignore_then(text::keyword("player"))
        .padded()
        .ignore_then(just('.'))
        .padded()
        .ignore_then(ident().padded())
        .padded()
        .then(
            actions_parser()
                .separated_by(just(';'))
                .allow_trailing()
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
                    action: first_upper(&name),
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
                .repeated()
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
                .repeated()
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

    choice((player_event, process, function))
}

pub fn argument_list<'a>() -> impl Parser<'a, &'a str, Vec<ItemData>, Err<Rich<'a, char>>> {
    arguments_parser()
        .separated_by(just(','))
        .collect::<Vec<ItemData>>()
        .padded()
        .delimited_by(just('('), just(')'))
        .padded()
}

/// This matches an ItemData to it's ID in the item type.
fn data_to_id(data: &ItemData) -> String {
    match data {
        ItemData::Number { .. } => "num".to_string(),
        ItemData::Text { .. } => "txt".to_string(),
        ItemData::VanillaItem { .. } => "item".to_string(),
        ItemData::Location { .. } => "loc".to_string(),
        _ => "var".to_string(),
    }
}

/// Converts the first letter of a String to uppercase.
fn first_upper(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
