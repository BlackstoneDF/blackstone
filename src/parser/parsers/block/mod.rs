use chumsky::{
    extra,
    input::ValueInput,
    prelude::Rich,
    primitive::{any, empty, todo},
    span::SimpleSpan,
    Parser,
};

use crate::{
    lexer::Token,
    parser::ast::{
        item::{event::EventDeceleration, Block},
        statement::{Statement, StatementType},
    },
};

mod statement;

pub fn parse<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Block, extra::Err<Rich<'input, Token>>> {
    empty().map_with_span(|t, s| Block {
        statements: vec![Statement {
            typ: StatementType::Assignment,
            span: SimpleSpan::new(0, 1),
        }],
        span: SimpleSpan::new(0, 1),
    })
}
