use chumsky::span::SimpleSpan;
use indexmap::IndexMap;

use crate::{lexer::Token, parser::ast::Span};

use super::{Expression, Identifier};

/// A literal is an expression that isn't a call
/// At least in the ast, something like var("name") is considered as a call
/// A literal is something like 1, "a", [2] or {a: 7}
pub enum Literal {
    Text(),
    Number(),
    Struct(StructLiteral),
    List(ListLiteral),
    Dict(DictionaryLiteral),
}

pub struct TextLiteral {
    
}

/// Example
/// ```bls
/// ["a", b, "c", 1, [2, 3]]
/// ```
pub struct ListLiteral {
    opening: Span,
    elements: Vec<(Expression, Option<Span>)>,
    closing: Span,
}

/// Example:
/// ```bls
/// Person {
///   name: "bob",
///   age: 3
/// }
/// ```
#[derive(Debug)]
pub struct StructLiteral {
    name: Identifier,
    opening: Span,
    pairs: Vec<StructKVPair>,
    closing: Span,
}

/// Example:
/// ```bls
/// name: "bob",
/// ```
#[derive(Debug)]
pub struct StructKVPair {
    key: Identifier,
    colon: Span,
    value_span: Expression,
    comma: Option<Span>,
}

pub struct DictionaryLiteral {
    opening: Span,
    pairs: Vec<DictKVPair>,
    closing: Span,
}

/// Example:
/// ```bls
/// "hello to": "bob",
/// ```
#[derive(Debug)]
pub struct DictKVPair {
    key: Expression,
    colon: Span,
    value_span: Expression,
    comma: Option<Span>,
}

// TODO: Move this because ast does not handle this
/// NOT FOR AST
#[derive(Debug)]
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

#[derive(Debug)]
pub struct StringLiteral {
    string: String,
    span: SimpleSpan,
}

#[derive(Debug)]
pub struct NumberLiteral {
    digits: String,
    span: SimpleSpan,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Variable {
    name: String,
    scope: VariableScope,
}

#[derive(Debug)]
pub struct PotionEffect {
    effect: String,
    duration: i8,
    amplifier: i8,
}

#[derive(Debug)]
pub struct GameValue {
    name: String,
    target: Target,
}

#[derive(Debug)]
pub struct Sound {
    sound: String,
    pitch: f64,
    volume: f64,
}

#[derive(Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
    pitch: f64,
    yaw: f64,
    is_block: bool,
}

#[derive(Debug)]
pub enum VariableScope {
    Local,
    Game,
    Save,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
