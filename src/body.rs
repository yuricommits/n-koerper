#[derive(Clone)]
pub struct Body {
    pub id: usize,
    pub pos: [f64; 2],
    pub vel: [f64; 2],
    pub mass: f64,
    pub radius: f64,
    pub color: [f32; 3],
}

impl Body {
    pub fn new(id: usize, x: f64, y: f64, vx: f64, vy: f64, mass: f64, color: [f32; 3]) -> Self {
        Self {
            id,
            pos: [x, y],
            vel: [vx, vy],
            mass,
            radius: (mass / 1e12).cbrt() * 0.005, // cube-root scale feels natural
            color,
        }
    }
}
