use self::{actions::{IRPlayerAction, IREntityAction}, events::{IREntityEvent, IRPlayerEvent}};

pub mod actions;
pub mod events;
pub mod values;

pub enum IRCodeBlock {
    PlayerAction(IRPlayerAction),
    PlayerEvent(IRPlayerEvent),
    EntityAction(IREntityAction),
    EntityEvent(IREntityEvent),
}