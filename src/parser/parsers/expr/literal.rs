use chumsky::{Parser, extra, prelude::Rich, span::SimpleSpan, input::ValueInput, primitive::todo};

use crate::{parser::ast::expr::Expression, lexer::Token};

pub fn parse<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Expression, extra::Err<Rich<'input, Token>>> {
    todo()
}