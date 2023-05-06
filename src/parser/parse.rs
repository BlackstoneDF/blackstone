use chumsky::{text, primitive::just, Parser, IterParser};

use crate::codegen::{block::Block, item_data::ItemData, item::Item};
use chumsky::extra::Err;
use chumsky::parser::Rich;

use super::{ident, datatypes::arguments_parser};

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Option<Block<'a>>>, Err<Rich<'a, char>>> {
    events_parser()
}

pub fn actions_parser<'a>() -> impl Parser<'a, &'a str, Vec<Option<Block<'a>>>, Err<Rich<'a, char>>> {
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

    player_action
}

pub fn events_parser<'a>() -> impl Parser<'a, &'a str, Vec<Option<Block<'a>>>, Err<Rich<'a, char>>> {
    let player_event = text::keyword("PlayerEvent")
        .ignore_then(just('('))
        .padded()
        .ignore_then(ident())
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
                Some(Block::EventDefinition {
                    block: "event",
                    action: name,
                }),
            );
            out
        });

    player_event
}

pub fn argument_list<'a>() -> impl Parser<'a, &'a str, Vec<ItemData>, Err<Rich<'a, char>>> {
    arguments_parser()
        .repeated()
        .collect::<Vec<ItemData>>()
        .padded()
        .delimited_by(just('('), just(')'))
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
