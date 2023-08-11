use chumsky::{
    extra,
    input::ValueInput,
    prelude::Rich,
    primitive::{any, todo},
    span::SimpleSpan,
    Parser,
};

use crate::{
    lexer::Token,
    parser::ast::item::{event::EventDeceleration, Block},
};

pub fn parse<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Block, extra::Err<Rich<'input, Token>>> {
    any().map_with_span(|t, s| Block { statements: todo!(), span: todo!() })
}
