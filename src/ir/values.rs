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
}