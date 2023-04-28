use std::sync::{Arc, Mutex};

use chumsky::prelude::*;

use crate::codegen::{block::Block, item::Item, item_data::ItemData};

pub fn parser() -> impl Parser<char, Vec<Option<Block<'static>>>, Error = Simple<char>> {
    let player_default: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    let ident = text::ident();

    let internal_commands = {
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
                None
            });

        type_command
    };

    let arguments = {
        let number = text::int::<_, Simple<char>>(10).map(|f| ItemData::Number {
            data: f
                .parse::<f32>()
                .expect("failed to f32 somehow - shouldnt be possible"),
        });

        let text = just::<char, char, Simple<char>>('"')
            .ignore_then(none_of('"').repeated())
            .then_ignore(just('"'))
            .map(|f| ItemData::Text {
                data: f.iter().collect(),
            });
        let _location = text::keyword("loc")
            .ignore_then(
                number
                    .separated_by(just(','))
                    .allow_trailing()
                    .padded()
                    .collect::<Vec<_>>()
                    .padded()
                    .delimited_by(just('('), just(')')),
            )
            .map(|f| if f.len() != 3 && f.len() != 5 {});

        text.or(number)
    };

    let actions = {
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
                    let mut id = "".to_string();
                    if let ItemData::Number { data: _ } = data {
                        id = "num".to_string();
                    }
                    if let ItemData::Text { data: _ } = data {
                        id = "txt".to_string();
                    }
                    items.push(Item {
                        id,
                        slot: slot.try_into().expect("failed ot convert to usize"),
                        item: data,
                    })
                }
                Some(Block::Code {
                    block: "player_action",
                    items,
                    action: f,
                    data: "",
                })
            });
        let game_action = text::keyword("plot")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then(
                arguments
                    
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
                    items.push(Item {
                        id: "item".to_string(),
                        slot: slot.try_into().expect("failed ot convert to usize"),
                        item: data,
                    })
                }
                Some(Block::Code {
                    block: "game_action",
                    items,
                    action: f,
                    data: "",
                })
            });
        player_action.or(game_action).or(internal_commands)
    };

    let events = {
        let player_event = text::keyword("plot")
            .ignore_then(just('.'))
            .ignore_then(text::keyword("playerEvent"))
            .ignore_then(just('.'))
            .ignore_then(ident)
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
            .map(|(name, args): (String, Vec<Option<Block>>)| {
                let mut out = args;
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
    };

    events
}
