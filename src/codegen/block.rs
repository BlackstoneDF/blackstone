use super::{misc::{BracketType, BracketDirection}, item::Item};

#[derive(Debug, Clone)]
pub struct Block<'a> {
    pub id: &'a str,
    pub block: &'a str,
    pub items: Option<Vec<Item>>,
    pub data: Option<&'a str>,
    pub action: Option<&'a str>,
    pub direct: Option<BracketDirection>,
    pub typ: Option<BracketType>,
}

impl Block<'_> {
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
}