use chumsky::span::SimpleSpan;

use self::literal::DataType;

use super::Span;

pub mod literal;

pub enum ExprType {
    Literal(DataType),
}

pub struct Expression {
    pub expr_type: ExprType,
    pub span: SimpleSpan
}

#[derive(Debug)]
pub struct Iden {
    pub string: String,
    pub span: Span
}