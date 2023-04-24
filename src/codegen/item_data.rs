use super::misc::VariableScope;

#[derive(Debug, Clone)]
pub enum ItemData {
    Variable { scope: VariableScope, name: String },
    Number { data: f32 },
    Text { data: String },
    VanillaItem { data: String },
    NoData,
}

impl ItemData {
    pub fn to_json(self) -> String {
        match self {
            Self::Variable { scope, name } => {
                format!(r#""data":{{"scope":{},"name":"{name}"}}"#, scope.to_json())
            }
            Self::Number { data } => {
                format!(r#""data":{{"name":"{data}"}}"#)
            }
            Self::Text { data } => {
                format!(r#""data":{{"name":"{data}"}}"#)
            }
            Self::VanillaItem { data } => format!(r#""data":{{"item":"{data}"}}"#),
            Self::NoData => format!(r#""NoData""#),
        }
    }
}