use chumsky::span::SimpleSpan;
use indexmap::IndexMap;

use super::Expression;

pub enum Literal {
    DataType(DataType),
    Struct(StructLiteral),
    List(Vec<Expression>),
}

pub struct StructLiteral {
    name: String,
    pairs: IndexMap<String, Expression>
}

// TODO impl
pub enum DataType {
    String(StringLiteral),
    Number(NumberLiteral),
    Variable(Variable),
    PotionEffect(PotionEffect),
    GameValue(GameValue),
    Particle(Particle),
    Sound(Sound),
    Vector(Vector),
    Location(Location),
    True,
    False,
}

pub struct StringLiteral {
    string: String,
    span: SimpleSpan
}

pub struct NumberLiteral {
    digits: String, 
    span: SimpleSpan
}

pub struct Particle {
    particle: String,
    amount: u32,
    size: f64,

    x: f64,
    y: f64,
    z: f64,

    vertical_spread: Option<i64>,
    horizontal_spread: Option<i64>,
    rgb: Option<Color>,
    color_variation: Option<i64>,
    size_variation: Option<i64>,
    motion_variation: Option<i64>,
    roll: Option<f64>,
}

pub struct Variable {
    name: String,
    scope: VariableScope,
}

pub struct PotionEffect {
    effect: String,
    duration: i8,
    amplifier: i8,
}

pub struct GameValue {
    name: String,
    target: Target,
}

pub struct Sound {
    sound: String,
    pitch: f64,
    volume: f64,
}

pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Location {
    x: f64,
    y: f64,
    z: f64,
    pitch: f64,
    yaw: f64,
    is_block: bool,
}

pub enum VariableScope {
    Local,
    Game,
    Save,
}

pub enum Target {
    Selection,
    Default,
    Killer,
    Damager,
    Shooter,
    Victim,
    Projectile,
    LastEntity,
    AllPlayers,
    AllEntities,
    AllMobs,
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn rgb_to_number(red: u8, green: u8, blue: u8) -> u32 {
        let red_shifted = (red as u32) << 16;
        let green_shifted = (green as u32) << 8;
        let blue_shifted = blue as u32;
        red_shifted | green_shifted | blue_shifted
    }
}
