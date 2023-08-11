use chumsky::span::SimpleSpan;

use self::event::EventDeceleration;

use super::{Span, statement::Statement};

pub mod event;

#[derive(Debug)]
pub struct Item {
    typ: ItemType,
    span: Span
}

#[derive(Debug)]
pub enum ItemType {
    EventDecl(EventDeceleration),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span 
}