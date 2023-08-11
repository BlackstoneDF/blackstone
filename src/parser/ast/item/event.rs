use chumsky::span::SimpleSpan;

use crate::parser::ast::{Span, expr::Iden};

use super::Block;

/// ```bls
/// event Iden(event) {}
/// ```
#[derive(Debug)]
pub struct EventDeceleration {
    pub event_token: Span,
    pub iden: Iden,
    pub paren_token: Span,
    pub event_iden: Iden,
    pub block: Block
}


// TODO: Add more visibilities
pub enum Visibility {
    Pub(PublicVis)
}

pub struct PublicVis {
    pub span: Span 
}