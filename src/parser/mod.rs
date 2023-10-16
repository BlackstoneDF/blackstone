use chumsky::{
    extra,
    input::{ValueInput},
    prelude::{Rich},
    span::SimpleSpan,
    Parser, select,
};
use logos::Lexer;

use crate::lexer::Token;

use self::ast::AstFile;

pub mod ast;
mod parsers;

pub fn parse_file(_tokens: Lexer<Token>, _src: &str) -> AstFile {
    /*
    let token_iter = tokens.spanned().map(|(tok, span)| {
        if let Ok(token) = tok {
            (token, span.into())
        } else {
            panic!("Lexer Error: {span:?}");
        }
    });

    let token_stream = Stream::from_iter(token_iter).spanned((src.len()..src.len()).into());

    let out = parsers::items::event::parse().parse(token_stream);
    println!("Result: {:?}", out);

    AstFile {
        items: todo!(),
        span: tokens.span().into(),
    }
    */
    todo!()
}

pub fn iden<'input, 'block, I: ValueInput<'input, Token = Token, Span = SimpleSpan>>(
) -> impl Parser<'input, I, Token, extra::Err<Rich<'input, Token>>> {
    select! {
        Token::Identifier(string) => Token::Identifier(string),
        Token::EventDef => Token::Identifier("event".to_string()),
        Token::FunctionDef => Token::Identifier("fn".to_string()),
        Token::ProcessDef => Token::Identifier("proc".to_string()),
        Token::MacroDef => Token::Identifier("macro".to_string()),
        Token::ChainDef => Token::Identifier("chain".to_string()),
    }
}