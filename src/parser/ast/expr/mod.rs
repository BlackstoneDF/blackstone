use chumsky::span::SimpleSpan;

use self::literal::DataType;

use super::Spanned;

pub mod literal;

#[derive(Debug)]
pub enum ExprType {
    Literal(DataType),
}

#[derive(Debug)]
pub struct Expression {
    pub expr_type: ExprType,
    pub span: SimpleSpan,
}

#[derive(Debug)]
pub struct Identifier {
    pub string: String,
    pub span: Spanned,
}
