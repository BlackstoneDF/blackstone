use chumsky::prelude::*;

use crate::codegen::{block::Block, item_data::ItemData};

pub fn parser() -> impl Parser<char, Vec<Block<'static>>, Error = Simple<char>> {
    let ident = text::ident();


    let actions = recursive(|actions| {
        let player_action = text::keyword("player")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then_ignore(just('('))
            .then_ignore(just(')'))
            .padded()
            .map(|f: String| Block::Code {
                block: "player_action",
                items: vec![],
                action: f,
                data: "",
            });
        let game_action = text::keyword("game")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then_ignore(just('('))
            .then_ignore(just(')'))
            .padded()
            .map(|f: String| Block::Code {
                block: "game_action",
                items: vec![],
                action: f,
                data: "",
            });
        player_action.or(game_action)
    });

    let events = {
        let player_event = text::keyword("player")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .padded()
            .then(
                actions.clone()
                    .separated_by(just(';'))
                    .allow_trailing()
                    .padded()
                    .collect::<Vec<_>>()
                    .padded()
                    .delimited_by(just('{'), just('}'))
                    .padded()
            )
            .padded()
            .map(|(name, args): (String, Vec<Block>)| {
                let mut out = args;
                out.insert(0, Block::EventDefinition { block: "event", action: name });
                println!("{out:#?}");
                out
            });

        player_event
    };

    events

}
