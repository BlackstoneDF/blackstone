use chumsky::{input::ValueInput, span::SimpleSpan, Parser, extra, prelude::Rich, primitive::todo};


use super::ast::ParsedStringType;

/*
pub fn parse<'input, 'block, I: ValueInput<'input, Token = StringToken, Span = SimpleSpan>>(
) -> impl Parser<'input, I, ParsedStringType, extra::Err<Rich<'input, StringToken>>> {
    todo()
}
*/