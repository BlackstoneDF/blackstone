use chumsky::{
    extra, input::ValueInput, prelude::Rich, primitive::{just, todo}, select, span::SimpleSpan, Parser,
};

use crate::{
    lexer::Token,
    parser::{ast::statement::Statement, iden, parsers::expr::expr},
    util::id2,
};

pub fn parse_assignment<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Statement, extra::Err<Rich<'input, Token>>> {
    select! {
        Token::Let,
        Token::Alias,
        Token::Sel
    }
    .map_with_span(id2)
    .then(iden())
    .map_with_span(id2)
    .then(just(Token::Equals))
    .map_with_span(id2)
    .then(expr())
    .map_with_span(id2)
    .then(just(Token::Semicolon))
    .map_with_span(|tokens, span| {
        Statement { typ: todo!(), span: todo!() }
    })
}
