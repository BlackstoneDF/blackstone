use super::Span;


#[derive(Debug)]
pub struct Statement {
    typ: StatementType,
    span: Span
}

#[derive(Debug)]
pub enum StatementType {
    Assignment,
    Call,
    If,
    Loop,
    

}