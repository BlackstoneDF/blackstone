enum IRValue {
    Text(String),
    Number(f32),
    Location(f32, f32, f32, f32, f32),
    Vector(f32, f32, f32),
    Particle {
        name: String,
        amount: u32,
        spread: (i32, i32),
        motion: (i32, i32, i32),
        motion_variation: u32,
    },
    Texts(Vec<IRValue>),
    Numbers(Vec<IRValue>),
    Locations(Vec<IRValue>),
    Vectors(Vec<IRValue>),
    Particles(Vec<IRValue>),
    
    OptionalText(Option<Box<IRValue>>),
    OptionalNumber(Option<Box<IRValue>>),
    OptionalLocation(Option<Box<IRValue>>),
    OptionalVector(Option<Box<IRValue>>),
    OptionalParticle(Option<Box<IRValue>>),

    OptionalTexts(Option<Vec<IRValue>>),
    OptionalNumbers(Option<Vec<IRValue>>),
    OptionalLocations(Option<Vec<IRValue>>),
    OptionalVectors(Option<Vec<IRValue>>),
    OptionalParticles(Option<Vec<IRValue>>)
}


