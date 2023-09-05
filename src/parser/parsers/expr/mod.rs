use chumsky::{extra::{self, Full}, prelude::Rich, span::SimpleSpan, input::ValueInput, Parser, primitive::todo, select};

use crate::{parser::ast::{expr::Expression, item::Block}, lexer::Token};

mod literal;

pub fn expr<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Expression, extra::Err<Rich<'input, Token>>> {
    todo()
}