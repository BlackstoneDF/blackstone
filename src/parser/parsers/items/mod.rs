pub mod event;

use chumsky::{
    extra,
    input::ValueInput,
    prelude::Rich,
    primitive::{choice, just, todo},
    recursive::recursive,
    span::SimpleSpan,
    Parser,
};

use crate::{lexer::Token, parser::ast::item::Item};

pub fn parse<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Vec<Item>, extra::Err<Rich<'input, Token>>> {
    todo()
}
