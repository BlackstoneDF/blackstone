use super::{misc::{BracketType, BracketDirection}, item::Item};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Block<'a> {
    CodeBlock {
        block: &'a str,
        items: Vec<Item>,
        action: &'a str,
        data: &'a str,
    },
    EventDefinition {
        block: &'a str,
        action: &'a str,
    },
    FunctionDefinition {
        block: &'a str,
        data: &'a str,
    },
    FunctionCall {
        block: &'a str,
        data: &'a str,
    },
    Bracket {
        direct: BracketDirection,
        typ: BracketType,
    }
}

#[allow(dead_code, unused)]
impl Block<'_> {
    pub fn to_json(self) -> String {
        match self {
            Block::CodeBlock { block, items, action, data } => { 
                let mut items_str = String::new();
                for item in items {
                    items_str.push_str(&item.to_json());
                    items_str.push(',');
                }
                items_str.pop();
                format!(r#"{{"id":"block","block":"{block}","args":{{"items":[{items_str}]}},"action":"{action}","data":"{data}"}}"#) 
            },
            Block::EventDefinition { block, action } => format!(r#"{{"id":"block","block":"{block}","action":"{action}","args":{{"items":[]}}}}"#),
            Block::Bracket { direct, typ } => format!(r#"{{"id":"bracket","direct":"{}","type":"{}"}}"#, direct.to_json(), typ.to_json()),
            Block::FunctionDefinition { block, data } => format!(r#"{{"id":"block","block":"{block}","data:{data}"}}"#),
            Block::FunctionCall { block, data } => format!(r#"{{"id":"block","block":"{block}",}}"#),
        }
    }
}