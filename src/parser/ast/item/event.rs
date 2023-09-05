use chumsky::span::SimpleSpan;

use crate::parser::ast::{Span, expr::Identifier};

use super::Block;

/// ```bls
/// event Iden(event) {}
/// ```
#[derive(Debug)]
pub struct EventDeceleration {
    pub event_token: Span,
    pub iden: Identifier,
    pub paren_token: Span,
    pub event_iden: Identifier,
    pub block: Block
}


// TODO: Add more visibilities
pub enum Visibility {
    Pub(PublicVis)
}

pub struct PublicVis {
    pub span: Span 
}