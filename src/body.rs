#[derive(Clone)]
pub struct Body {
    pub pos: [f64; 2],
    pub vel: [f64; 2],
    pub mass: f64,
}

impl Body {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, mass: f64) -> Self {
        Self {
            pos: [x, y],
            vel: [vx, vy],
            mass,
        }
    }
}
