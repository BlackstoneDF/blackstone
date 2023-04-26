use super::{actions::*, events::*};

#[allow(dead_code)]
pub enum IRCodeBlock {
    PlayerAction(IRPlayerAction),
    PlayerEvent(IRPlayerEvent),
    EntityAction(IREntityAction),
    EntityEvent(IREntityEvent),
}
