use crate::body::Body;

const G: f64 = 6.674e-11;
const SOFTENING: f64 = 1e-4;

pub struct Simulation {
    pub bodies: Vec<Body>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            bodies: vec![
                Body::new(0.0, 0.05, 0.0, 5.0, 1e12), // orbit around centre
                Body::new(0.0, -0.05, 0.0, -5.0, 1e12),
                Body::new(0.0, 0.0, 0.0, 0.0, 5e13), // heavy centre
            ],
        }
    }
}
