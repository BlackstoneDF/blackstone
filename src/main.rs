#![allow(warnings)]
use std::ops::Index;

use chumsky::{span::SimpleSpan, primitive::{choice, just}};
use indexmap::IndexMap;
use logos::Logos;
use template::lexer::{StringToken, StringLexer};

pub mod lexer;
pub mod parser;
pub mod util;
pub mod template;

const SRC: &str = "hello ${if ()}";

fn main() {
    let a: Vec<_> = StringLexer::new(SRC).map(|it| it.unwrap()).collect();
    println!("{:#?}", a);

    /*
    let lexer = lexer::Token::lexer(SRC);
    println!("Tokens: {:?}", lexer.clone().collect::<Vec<_>>());

    let output = parser::parse_file(lexer, SRC);
    println!("AstFile: {:#?}", output);
    */

}

