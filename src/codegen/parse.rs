use chumsky::prelude::*;

use crate::codegen::{block::Block, item_data::ItemData};

pub fn parser() -> impl Parser<char, Block<'static>, Error = Simple<char>> {
    let ident = text::ident();


    let player_action = text::keyword("playerAction")
        .ignore_then(just('.'))
        .ignore_then(ident)
        .then_ignore(just('('))
        .then_ignore(just(')'))
        .map(|f: String| {
            Block::Code {
                block: "player_action",
                items: vec![],
                action: f,
                data: "",
            }
        });
    player_action
}
