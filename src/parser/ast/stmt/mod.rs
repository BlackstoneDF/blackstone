use super::{Spanned, expr::Expression};


#[derive(Debug)]
pub struct Statement {
    pub typ: Spanned<StatementType>,
}

#[derive(Debug)]
pub enum StatementType {
    Assignment,
    Expr(Expression),
    Call,
    If,
    Loop,

}
