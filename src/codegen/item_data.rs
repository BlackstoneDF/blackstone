use super::misc::VariableScope;

#[derive(Debug, Clone)]
pub enum ItemData {
    Variable { scope: VariableScope, name: String },
    Number { data: f32 },
    Text { data: String },
    VanillaItem { data: String },
    Sound { sound: String, pitch: f32, vol: f32 },
    Vector {x: f32, y: f32, z: f32},
    Location {x: f32, y: f32, z: f32, pitch: f32, yaw: f32},
    Potion {effect: String, dur: u32, amp: u32},
    NoData,
}

impl ItemData {
    pub fn to_json(&self) -> String {
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
            Self::Sound { sound, pitch, vol } => {
                format!(r#""data":{{"sound":"{sound}","pitch":"{pitch}","vol":"{vol}"}}"#)
            }
            Self::VanillaItem { data } => format!(r#""data":{{"item":"{data}"}}"#),
            Self::Vector {x, y, z} => {
                format!(r#""data":{{"x":"{x}","y":"{y}","z":"{z}"}}"#)
            }
            Self::Location {x, y, z, pitch, yaw} => {
                format!(r#""data":{{"x":"{x}","y":"{y}","z":"{z}", "pitch":"{pitch}", "yaw":"{yaw}"}}"#)
            }
            Self::Potion {effect, dur, amp} => {
                format!(r#""data":{{"effect":"{effect}","dur":"{dur}","amp":"{amp}"}}"#)
            }
            Self::NoData => format!(r#""NoData""#),
        }
    }
}