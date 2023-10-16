use self::bind::EventDeceleration;

use super::{Spanned, stmt::Statement};

pub mod bind;
pub mod utils;

#[derive(Debug)]
pub struct Item {
    typ: ItemType,
    span: Spanned
}

#[derive(Debug)]
pub enum ItemType {
    EventDecl(EventDeceleration),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Spanned
}

