use chumsky::{prelude::Simple, Parser, recursive::{recursive, Recursive}, text::{ident, self, TextParser}, primitive::just};

use crate::codegen::{block::Block, item_data::ItemData, item::Item};

use super::datatypes::arguments_parser;

pub fn parser() -> impl Parser<char, Vec<Option<Block<'static>>>, Error = Simple<char>> {
    actions_parser()
}

pub fn actions_parser() -> impl Parser<char, Vec<Option<Block<'static>>>, Error = Simple<char>> {
    let actions = recursive(
        |actions: Recursive<char, Vec<Option<Block>>, Simple<char>>| {
            let operation = just::<char, &str, Simple<char>>("=")
                .or(just("+"))
                .or(just("-"))
                .or(just("*"))
                .or(just("/"))
                .or(just("%"));

            let player_action = text::keyword("player")
                .ignore_then(just('.'))
                .ignore_then(ident())
                .then(
                    arguments_parser()
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
                        target: "Selection",
                        inverted: "",
                        sub_action: String::new(),
                    })]
                });

            player_action
        },
    );
    actions
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