use chumsky::{
    extra,
    input::{Stream, ValueInput},
    prelude::{Input, Rich},
    primitive::{any, todo, Just, choice, Choice, just},
    span::SimpleSpan,
    Parser,
};
use logos::Lexer;

use crate::lexer::Token;

use self::ast::AstFile;

pub mod ast;
mod parsers;

pub fn parse(tokens: Lexer<Token>, src: &str) -> AstFile {
    let token_iter = tokens.spanned().map(|(tok, span)| {
        if let Ok(token) = tok {
            (token, span.into())
        } else {
            panic!("ERROR {span:?}");
        }
    });

    let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

    let out = parsers::items::event::parse().parse(token_stream);
    println!("{:?}", out);
    AstFile {
        items: todo!(),
        span: tokens.span().into(),
    }
}

/* 
pub fn expr<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Pain, extra::Err<Rich<'input, Token>>> {
    choice((
        just(Token::Identifier),

        just(Token::If),
        just(Token::FunctionDef),
        just(Token::ProcessDef),
        just(Token::MacroDef),
        just(Token::EventDef)
    ))
}
*/