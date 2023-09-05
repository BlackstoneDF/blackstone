use super::Span;


#[derive(Debug)]
pub struct Statement {
    pub typ: StatementType,
    pub span: Span
}

#[derive(Debug)]
pub enum StatementType {
    Assignment,
    Call,
    If,
    Loop,
    

}