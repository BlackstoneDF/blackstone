use strum::IntoEnumIterator;

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IRPlayerAction {

}

impl IRPlayerAction {
    fn parse_player_action(code: &str) -> Option<IRPlayerAction> {
        if !code.starts_with("playerAction.") {
            return None;
        }
        let code = code.trim_start_matches("playerAction.");
        IRPlayerAction::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}


#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IRGameAction {
    
}

impl IRGameAction {
    fn parse_player_action(code: &str) -> Option<IRGameAction> {
        if !code.starts_with("playerAction.") {
            return None;
        }
        let code = code.trim_start_matches("gameAction.");
        IRGameAction::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}

#[derive(strum_macros::EnumIter, strum_macros::Display, PartialEq)]
pub enum IREntityAction {
    
}

impl IREntityAction {
    fn parse_player_action(code: &str) -> Option<IREntityAction> {
        if !code.starts_with("entityAction.") {
            return None;
        }
        let code = code.trim_start_matches("gameAction.");
        IREntityAction::iter().find(|event| event.to_string().eq_ignore_ascii_case(code))
    }
}