#[derive(PartialEq, Default)]
pub struct Text(pub String);
#[derive(PartialEq, Default)]
pub struct Number(pub f32);
#[derive(PartialEq, Default)]
pub struct Location(pub f32, pub f32, pub f32, pub f32, pub f32);
#[derive(PartialEq, Default)]
pub struct Vector(pub f32, pub f32, pub f32);
#[derive(PartialEq, Default)]
pub struct Particle {
    pub name: String,
    pub amount: u32,
    pub spread: (i32, i32),
    pub motion: (i32, i32, i32),
    pub motion_variation: u32,
}
#[derive(PartialEq, Default)]
pub struct Texts(pub Vec<Text>);
#[derive(PartialEq, Default)]
pub struct Numbers(pub Vec<Number>);
#[derive(PartialEq, Default)]
pub struct Locations(pub Vec<Location>);
#[derive(PartialEq, Default)]
pub struct Vectors(pub Vec<Vector>);
#[derive(PartialEq, Default)]
pub struct Particles(pub Vec<Particle>);
#[derive(PartialEq, Default)]
pub struct OptionalText(pub Option<Text>);
#[derive(PartialEq, Default)]
pub struct OptionalNumber(pub Option<Number>);
#[derive(PartialEq, Default)]
pub struct OptionalLocation(pub Option<Location>);
#[derive(PartialEq, Default)]
pub struct OptionalVector(pub Option<Vector>);
#[derive(PartialEq, Default)]
pub struct OptionalParticle(pub Option<Particle>);
#[derive(PartialEq, Default)]
pub struct OptionalTexts(pub Option<Texts>);
#[derive(PartialEq, Default)]
pub struct OptionalNumbers(pub Option<Numbers>);
#[derive(PartialEq, Default)]
pub struct OptionalLocations(pub Option<Locations>);
#[derive(PartialEq, Default)]
pub struct OptionalVectors(pub Option<Vectors>);
#[derive(PartialEq, Default)]
pub struct OptionalParticles(pub Option<Particles>);
