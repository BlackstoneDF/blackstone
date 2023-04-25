struct Text(String);
struct Number(f32);
struct Location(f32, f32, f32, f32, f32);
struct Vector(f32, f32, f32);
struct Particle {
    name: String,
    amount: u32,
    spread: (i32, i32),
    motion: (i32, i32, i32),
    motion_variation: u32,
}
struct Texts(Vec<Text>);
struct Numbers(Vec<Number>);
struct Locations(Vec<Location>);
struct Vectors(Vec<Vector>);
struct Particles(Vec<Particle>);
struct OptionalText(Option<Text>);
struct OptionalNumber(Option<Number>);
struct OptionalLocation(Option<Location>);
struct OptionalVector(Option<Vector>);
struct OptionalParticle(Option<Particle>);
struct OptionalTexts(Option<Texts>);
struct OptionalNumbers(Option<Numbers>);
struct OptionalLocations(Option<Locations>);
struct OptionalVectors(Option<Vectors>);
struct OptionalParticles(Option<Particles>);
