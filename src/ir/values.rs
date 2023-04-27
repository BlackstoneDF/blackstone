use crate::codegen;

#[derive(PartialEq, Default, Debug)]
pub struct Text(String);
#[derive(PartialEq, Default, Debug)]
pub struct Variable(String);
#[derive(PartialEq, Default, Debug)]
pub struct Number(f32);
#[derive(PartialEq, Default, Debug)]
pub struct Location(f32, f32, f32, f32, f32);
#[derive(PartialEq, Default, Debug)]
pub struct Vector(f32, f32, f32);
#[derive(PartialEq, Default, Debug)]
pub struct Particle {
    name: String,
    amount: u32,
    spread: (i32, i32),
    motion: (i32, i32, i32),
    motion_variation: u32,
}
#[derive(PartialEq, Default, Debug)]
pub struct Sound {
    sound: String,
    pitch: f32,
    vol: f32,
}
#[derive(PartialEq, Default, Debug)]
pub struct Potion {
    effect: String,
    dur: u32,
    amp: u32,
}
#[derive(PartialEq, Default, Debug)]
pub struct Item(codegen::item::Item);
#[derive(PartialEq, Default, Debug)]
pub struct Texts(Vec<Text>);
#[derive(PartialEq, Default, Debug)]
pub struct Variables(Vec<Variable>);
#[derive(PartialEq, Default, Debug)]
pub struct Numbers(Vec<Number>);
#[derive(PartialEq, Default, Debug)]
pub struct Locations(Vec<Location>);
#[derive(PartialEq, Default, Debug)]
pub struct Vectors(Vec<Vector>);
#[derive(PartialEq, Default, Debug)]
pub struct Particles(Vec<Particle>);
#[derive(PartialEq, Default, Debug)]
pub struct Items(Vec<Item>);
#[derive(PartialEq, Default, Debug)]
pub struct Sounds(Vec<Sound>);
#[derive(PartialEq, Default, Debug)]
pub struct Potions(Vec<Potion>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalText(Option<Text>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalVariable(Option<Variable>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalNumber(Option<Number>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalLocation(Option<Location>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalVector(Option<Vector>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalParticle(Option<Particle>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalItem(Option<Item>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalSound(Option<Sound>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalPotion(Option<Potion>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalTexts(Option<Texts>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalVariables(Option<Variables>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalNumbers(Option<Numbers>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalLocations(Option<Locations>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalVectors(Option<Vectors>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalParticles(Option<Particles>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalItems(Option<Items>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalSounds(Option<Sounds>);
#[derive(PartialEq, Default, Debug)]
pub struct OptionalPotions(Option<Potions>);
#[derive(PartialEq, Default, Debug)]
pub struct BoolTag(bool);
#[derive(PartialEq, Default, Debug)]
pub struct StringTag(String);
