use std::any::type_name;

use chumsky::{
    extra,
    input::ValueInput,
    prelude::Rich,
    primitive::{choice, just},
    select,
    span::SimpleSpan,
    Parser,
};

use crate::{
    lexer::Token,
    parser::{
        ast::{expr::Identifier, item::event::EventDeceleration},
        parsers::block, iden,
    },
    util::id2,
};

pub fn parse<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, EventDeceleration, extra::Err<Rich<'input, Token>>> {
    // event Join(event) {}

    /*
            just(Token::Identifier),
            just(Token::EventDef),
            just(Token::FunctionDef),
            just(Token::MacroDef),
            just(Token::ProcessDef),
    */
    just(Token::EventDef)
        .then(iden())
        .then(just(Token::OpenParen))
        .then(iden())
        .then(just(Token::CloseParen))
        .then(block::parse().delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)))
        .map_with_span(|tokens, span| {
            println!("Yes: {:?}", tokens);
            EventDeceleration {
                event_token: todo!(),
                iden: Identifier {
                    string: todo!(),
                    span: todo!(),
                },
                paren_token: todo!(),
                event_iden: todo!(),
                block: todo!(),
            }
        })
}
