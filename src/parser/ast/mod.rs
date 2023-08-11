use chumsky::span::SimpleSpan;

use self::item::Item;

pub mod expr;
pub mod item;
pub mod statement;

// To swap out when we need file ids and stuff
type Span = SimpleSpan;

#[derive(Debug)]
pub struct AstFile {
    pub items: Vec<Item>,
    pub span: Span,
}

