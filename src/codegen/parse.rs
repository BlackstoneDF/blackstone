use chumsky::prelude::*;

use crate::codegen::{block::Block, item_data::ItemData};

pub fn parser() -> impl Parser<char, Block<'static>, Error = Simple<char>> {
    recursive(|_block| {
        let ident = text::ident();

        let player_event = text::keyword("playerEvent")
            .ignore_then(just(':'))
            .ignore_then(just(':'))
            .ignore_then(ident)
            .then_ignore(just('{'))
            .then_ignore(just('}'))
            .map(|f: String| Block::EventDefinition {
                block: "event",
                action: f,
            });

        let player_action = text::keyword("playerAction")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then_ignore(just('('))
            .then_ignore(just(')'))
            .then_ignore(just(';'))
            .map(|f: String| Block::Code {
                block: "player_action",
                items: vec![],
                action: f,
                data: "",
            });

        let game_action = text::keyword("gameAction")
            .ignore_then(just('.'))
            .ignore_then(ident)
            .then_ignore(just('('))
            .then_ignore(just(')'))
            .then_ignore(just(';'))
            .map(|f: String| Block::Code {
                block: "game_action",
                items: vec![],
                action: f,
                data: "",
            });

        choice((player_event, player_action, game_action))
    })
}
