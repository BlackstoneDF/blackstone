use crate::codegen;

#[derive(PartialEq, Default)]
pub struct Text(String);
#[derive(PartialEq, Default)]
pub struct Number(f32);
#[derive(PartialEq, Default)]
pub struct Location(f32, f32, f32, f32, f32);
#[derive(PartialEq, Default)]
pub struct Vector(f32, f32, f32);
#[derive(PartialEq, Default)]
pub struct Particle {
    name: String,
    amount: u32,
    spread: (i32, i32),
    motion: (i32, i32, i32),
    motion_variation: u32,
}
#[derive(PartialEq, Default)]
pub struct Sound {
    sound: String,
    pitch: f32,
    vol: f32,
}
#[derive(PartialEq, Default)]
pub struct Potion {
    effect: String,
    dur: u32,
    amp: u32,
}
#[derive(PartialEq, Default)]
pub struct Item(codegen::item::Item);
#[derive(PartialEq, Default)]
pub struct Texts(Vec<Text>);
#[derive(PartialEq, Default)]
pub struct Numbers(Vec<Number>);
#[derive(PartialEq, Default)]
pub struct Locations(Vec<Location>);
#[derive(PartialEq, Default)]
pub struct Vectors(Vec<Vector>);
#[derive(PartialEq, Default)]
pub struct Particles(Vec<Particle>);
#[derive(PartialEq, Default)]
pub struct Items(Vec<Item>);
#[derive(PartialEq, Default)]
pub struct Sounds(Vec<Sound>);
#[derive(PartialEq, Default)]
pub struct Potions(Vec<Potion>);
#[derive(PartialEq, Default)]
pub struct OptionalText(Option<Text>);
#[derive(PartialEq, Default)]
pub struct OptionalNumber(Option<Number>);
#[derive(PartialEq, Default)]
pub struct OptionalLocation(Option<Location>);
#[derive(PartialEq, Default)]
pub struct OptionalVector(Option<Vector>);
#[derive(PartialEq, Default)]
pub struct OptionalParticle(Option<Particle>);
#[derive(PartialEq, Default)]
pub struct OptionalItem(Option<Item>);
#[derive(PartialEq, Default)]
pub struct OptionalSound(Option<Sound>);
#[derive(PartialEq, Default)]
pub struct OptionalPotion(Option<Potion>);
#[derive(PartialEq, Default)]
pub struct OptionalTexts(Option<Texts>);
#[derive(PartialEq, Default)]
pub struct OptionalNumbers(Option<Numbers>);
#[derive(PartialEq, Default)]
pub struct OptionalLocations(Option<Locations>);
#[derive(PartialEq, Default)]
pub struct OptionalVectors(Option<Vectors>);
#[derive(PartialEq, Default)]
pub struct OptionalParticles(Option<Particles>);
#[derive(PartialEq, Default)]
pub struct OptionalItems(Option<Items>);
#[derive(PartialEq, Default)]
pub struct OptionalSounds(Option<Sounds>);
#[derive(PartialEq, Default)]
pub struct OptionalPotions(Option<Potions>);
