#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IRBracket {
    NormOpen,
    NormClose,
    RepeatOpen,
    RepeatClose,
}