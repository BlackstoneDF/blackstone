use super::misc::VariableScope;

/// Represents a literal block of code in a DF code line.
/// See individual variant documentation for more information.
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(dead_code, unused)]
pub enum ItemData {
    /// A DF variable with a given scope and name. Variables with the same name but different scopes do not conflict.
    ///   - VariableScope `scope` - The scope of the variable. Can be Game, Save, or Local.
    ///   - String `name` - The name of the variable.
    Variable {
        scope: VariableScope,
        name: String,
    },
    /// A DF Number type.
    ///   - f32 `data` - The underlying numeric value.
    Number {
        data: f32,
    },
    /// A DF String type.
    ///   - String `data` - The underlying String value.
    Text {
        data: String,
    },
    /// A vanilla Minecraft item.
    ///   - String `data` - Associated data with the item.
    VanillaItem {
        data: String,
    },
    /// A DF Sound type.
    ///   - String `sound` - the name of the sound
    ///   - f32 `pitch` - the pitch of the sound
    ///   - f32 `vol` - the volume of the sound
    Sound {
        sound: String,
        pitch: f32,
        vol: f32,
    },
    /// A DF Vector type.
    ///   - f32 `x` - X component
    ///   - f32 `y` - Y component
    ///   - f32 `z` - Z component
    Vector {
        x: f32,
        y: f32,
        z: f32,
    },
    /// A DF Location type.
    ///   - f32 `x` - X coordinate
    ///   - f32 `y` - Y coordinate
    ///   - f32 `z` - Z coordinate
    ///   - f32 `pitch` - The pitch (between 90.0 and -90.0)
    ///   - f32 `yaw` - The yaw (between 180 and -180)
    Location {
        x: f32,
        y: f32,
        z: f32,
        pitch: f32,
        yaw: f32,
    },
    /// A DF Potion type.
    ///   - String `effect` - The effect of the potion (speed, etc.)
    ///   - u32 `dur` - The duration of the potion (in ticks)
    ///   - u32 `amp` - The amplitude (level) of the potion
    Potion {
        effect: String,
        dur: u32,
        amp: u32,
    },
    /// A DF Particle type.
    ///   - String `name` - The name of the particle
    ///   - u32 `amount` - The amount of particles
    ///   - (i32, i32) `spread` - The spread of particles
    ///   - (i32, i32, i32) `motion` - The motion of particles
    ///   - u32 `motion_variation` - The variation of motion
    Particle {
        name: String,
        amount: u32,
        spread: (i32, i32),
        motion: (i32, i32, i32),
        motion_variation: u32,
    },
    #[default]
    NoData,
}

#[allow(dead_code, unused)]
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
            Self::Vector { x, y, z } => {
                format!(r#""data":{{"x":"{x}","y":"{y}","z":"{z}"}}"#)
            }
            Self::Location {
                x,
                y,
                z,
                pitch,
                yaw,
            } => {
                format!(
                    r#""data":{{"x":"{x}","y":"{y}","z":"{z}", "pitch":"{pitch}", "yaw":"{yaw}"}}"#
                )
            }
            Self::Potion { effect, dur, amp } => {
                format!(r#""data":{{"effect":"{effect}","dur":"{dur}","amp":"{amp}"}}"#)
            }
            Self::Particle {
                amount,
                spread,
                motion,
                motion_variation,
                name,
            } => {
                format!(
                    r#""data":{{"name":"{name}","amount":"{amount}","spread":"({}, {})", "motion":"({}, {}, {})", "motion_variation":"{motion_variation}"}}"#,
                    spread.0, spread.1, motion.0, motion.1, motion.2
                )
            }
            Self::NoData => r#""NoData""#.to_string(),
        }
    }
}
