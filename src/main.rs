#![allow(warnings)]
use chumsky::{span::SimpleSpan, primitive::{choice, just}};
use logos::Logos;

pub mod lexer;
pub mod parser;
pub mod util;

const SRC: &str = r#"
event Join(event) {}
"#;

fn main() {
    let lexer = lexer::Token::lexer(SRC);
    println!("Tokens: {:?}", lexer.clone().collect::<Vec<_>>());

    let output = parser::parse(lexer, SRC);
    println!("AstFile: {:#?}", output);

}

