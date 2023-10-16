use std::fmt::Debug;

use chumsky::span::SimpleSpan;

use self::item::Item;

pub mod expr;
pub mod item;
pub mod stmt;
pub mod utils;

// To swap out when we need file ids and stuff
// type Spanned = SimpleSpan;

#[derive(Debug)]
pub struct AstFile {
    pub items: Vec<Item>,
    pub span: Spanned,
}

#[derive(Debug)]
pub struct Spanned<D = (), C = ()> where SimpleSpan<u32, C>: Debug {
    span: SimpleSpan<u32, C>,
    data: D,
}
