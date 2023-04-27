use super::{actions::*, events::*};

#[allow(dead_code)]
#[derive(Debug)]
pub enum IRCodeBlock {
    PlayerEvent(
        IRPlayerEvent,
        Box<Self>, // requires Block
    ),
    EntityEvent(
        IREntityEvent,
        Box<Self>, // requires Block
    ),
    Function(
        String,    // name
        Box<Self>, // code, requires Block
    ),
    Block(Vec<Self>),
    PlayerAction(IRPlayerAction),
    EntityAction(IREntityAction),
}
