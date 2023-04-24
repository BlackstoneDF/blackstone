use super::{misc::{BracketType, BracketDirection}, item::Item};


/* pub struct Block<'a> {
    pub id: &'a str,
    pub block: &'a str,
    pub items: Option<Vec<Item>>,
    pub data: Option<&'a str>,
    pub action: Option<&'a str>,
    pub direct: Option<BracketDirection>,
    pub typ: Option<BracketType>,
} */

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

/* impl Block<'_> {
    pub fn to_json(self) -> String {
        let mut out_str = String::new();
        out_str.push_str(
            format!(
                r#"{{"id":"{}","block":"{}","args":{{"items":"#,
                self.id, self.block
            )
            .as_str(),
        );
        // left: {}]}}}}
        if let Some(v) = self.items {
            out_str.push_str("[");
            for item in v {
                out_str.push_str(format!("{}", item.to_json()).as_str());
                out_str.push(',');
            }
            out_str.pop();
            out_str.push_str("]");
        } else {
            out_str.push_str("[]");
        }
        out_str.push_str("}");

        // left: ,}}
        if let Some(v) = self.data {
            out_str.push_str(format!(r#","data":{v}""#).as_str());
        }
        if let Some(v) = self.action {
            out_str.push_str(format!(r#","action":"{v}""#).as_str());
        }
        if let Some(v) = self.direct {
            let json = v.to_json();
            out_str.push_str(format!(r#","direct":"{json}""#).as_str());
        }
        if let Some(v) = self.typ {
            let json = v.to_json();
            out_str.push_str(format!(r#","type":"{json}""#).as_str());
        }
        out_str.push_str("}");
        out_str
    }
} */

/*
{
  "blocks": [
    {
      "id": "block",
      "block": "event",
      "args": {
        "items": []
      },
      "action": "PlaceBlock"
    },
    {
      "id": "block",
      "block": "player_action",
      "args": {
        "items": []
      },
      "action": "SendMessageSeq"
    }
  ]
}

 */

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
            Block::Bracket { direct, typ } => format!(r#"{{}}"#),
            Block::FunctionDefinition { block, data } => format!(r#"{{"id":"block","block":"{block}","data:{data}"}}"#),
            Block::FunctionCall { block, data } => format!(r#"{{"id":"block","block":"{block}",}}"#),
        }
    }
}