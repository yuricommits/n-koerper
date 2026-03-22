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
    pub fn step(&mut self, dt: f64) {
        let n = self.bodies.len();
        let mut forces = vec![[0.0_f64; 2]; n];

        for i in 0..n {
            for j in (i + 1)..n {
                let dx = self.bodies[j].pos[0] - self.bodies[i].pos[0];
                let dy = self.bodies[j].pos[1] - self.bodies[i].pos[1];
                let dist2 = dx * dx + dy * dy + SOFTENING * SOFTENING;
                let dist = dist2.sqrt(); // Pythagoras: dx² + dy²
                let f = G * self.bodies[i].mass * self.bodies[j].mass / dist2; // F = G * m1 * m2 / r²

                let fx = f * dx / dist; // unit vector pointing from i toward j
                let fy = f * dy / dist; // dividing by the distance normalizes it to length 1

                forces[i][0] += fx;
                forces[i][1] += fy;
                forces[j][0] -= fx; // Newton's 3rd law
                forces[j][1] -= fy;
            }
        }
        // Update velocities and positions
        for (i, body) in self.bodies.iter_mut().enumerate() {
            let ax = forces[i][0] / body.mass; // Newton's 2nd law: F = ma, so a = F/m
            let ay = forces[i][1] / body.mass; // A massive body accelerates less from the same force
            body.vel[0] += ax * dt; // Acceleration changes velocity over time:
            body.vel[1] += ay * dt; // `vel += acceleration * dt`
            body.pos[0] += body.vel[0] * dt;
            body.pos[1] += body.vel[1] * dt;
        }
    }
}

/*
pair distance → gravitational force magnitude → force direction
→ split force between both bodies (Newton 3rd)
→ force / mass = acceleration
→ acceleration * dt = velocity change
→ velocity * dt = position change
 */
