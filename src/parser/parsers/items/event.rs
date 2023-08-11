use std::any::type_name;

use chumsky::{
    extra,
    input::ValueInput,
    prelude::Rich,
    primitive::{choice, just},
    span::SimpleSpan,
    Parser, select,
};

use crate::{
    lexer::Token,
    parser::{
        ast::{expr::Iden, item::event::EventDeceleration},
        parsers::block,
    },
    util::id2,
};

pub fn parse<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, EventDeceleration, extra::Err<Rich<'input, Token>>> {
    // event Join(event) {}

    let expr_parse = select! {
        Token::EventDef
    };
    /* 
            just(Token::Identifier),
            just(Token::EventDef),
            just(Token::FunctionDef),
            just(Token::MacroDef),
            just(Token::ProcessDef),
    */
    just(Token::EventDef)
        .then(just(Token::Identifier))
        .then(just(Token::OpenParen))
        .then(expr_parse)
        .or_not()
        .then(just(Token::CloseParen))
        .then(block::parse().delimited_by(just(Token::OpenBrace), just(Token::CloseBrace)))
        .then(just(Token::CloseBrace))
        .map_with_span(|tokens, span| {
            println!("{:?}", tokens);
            EventDeceleration {
                event_token: todo!(),
                iden: Iden {
                    string: todo!(),
                    span: todo!(),
                },
                paren_token: todo!(),
                event_iden: todo!(),
                block: todo!(),
            }
        })
}
