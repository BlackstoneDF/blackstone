#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IRBracket {
    NormOpen,
    NormClose,
    RepeatOpen,
    RepeatClose,
}

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum PlayerConditions {

}

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum EntityConditions {

}

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum GameConditions {

}