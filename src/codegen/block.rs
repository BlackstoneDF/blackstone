use super::{
    item::Item,
    misc::{BracketDirection, BracketType},
};

/// Represents a literal block of code in a DF code line.
/// See individual variant documentation for more information.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)] // allowing this, CodeBlock is just a catch all for any non-event or non-function / non-process
pub enum Block<'a> {
    /// Defines a single code action (e.g. PlayerAction or Control).
    ///   - &'a str `block`: The associated block (cobblestone for PlayerAction, etc.)
    ///   - Vec<Item> `items`: The items contained within the chest above the block (empty if none)
    ///   - &'a str `action`: The associated action (e.g. SendMessage or Wait)
    ///   - &'a str `data`: Any extraneous data
    Code {
        block: &'a str,
        items: Vec<Item>,
        action: &'a str,
        data: &'a str,
    },
    /// Defines the definition of an event (either PlayerEvent or EntityEvent)
    ///   - &'a str `block`: The associated block (Diamond Block for PlayerEvent, Gold Block for EntityEvent)
    ///   - &'a str `action`: The associated action (e.g. join or killPlayer)
    EventDefinition { block: &'a str, action: &'a str },
    /// Defines a function definition
    ///   - &'a str `block`: The associated block
    ///   - &'a str `data`: Associated data (name, etc.)
    FunctionDefinition { block: &'a str, data: &'a str },
    /// Defines a call to a given function
    ///   - &'a str `block`: The associated block
    ///   - &'a str `data`: Associated data (name, etc.)
    FunctionCall { block: &'a str, data: &'a str },
    /// Defines a bracket block (piston)
    ///   - BracketDirection `direct` - the direction of the bracket (opening or closing)
    ///   - BracketType `type` - the type of the bracket (Normal/Piston or Repeat/Sticky Piston)
    Bracket {
        direct: BracketDirection,
        typ: BracketType,
    },
}

#[allow(dead_code, unused)]
impl Block<'_> {
    /// converts self to a workable json String
    pub fn to_json(&self) -> String {
        match self {
            Block::Code {
                block,
                items,
                action,
                data,
            } => {
                let mut items_str = String::new();
                for item in items {
                    items_str.push_str(item.to_json().as_str());
                    items_str.push(',');
                }
                items_str.pop();
                format!(
                    r#"{{"id":"block","block":"{block}","args":{{"items":[{items_str}]}},"action":"{action}","data":"{data}"}}"#
                )
            }
            Block::EventDefinition { block, action } => format!(
                r#"{{"id":"block","block":"{block}","action":"{action}","args":{{"items":[]}}}}"#
            ),
            Block::Bracket { direct, typ } => format!(
                r#"{{"id":"bracket","direct":"{}","type":"{}"}}"#,
                direct.to_json(),
                typ.to_json()
            ),
            Block::FunctionDefinition { block, data } => {
                format!(r#"{{"id":"block","block":"{block}","data:{data}"}}"#)
            }
            Block::FunctionCall { block, data } => {
                format!(r#"{{"id":"block","block":"{block}",}}"#)
            }
        }
    }
}
