use crate::body::Body;
use rand::RngExt;

const G: f64 = 6.674e-11;
const SOFTENING: f64 = 1e-3; // prevent infinite force when bodies overlap

pub struct Simulation {
    pub bodies: Vec<Body>,
}

impl Simulation {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let central_mass: f64 = rng.random_range(1e13..1e14); // Randomize the central heavy mass: 1×10¹³ → 1×10¹⁴
        let n_orbiting: usize = rng.random_range(2..=10); // Randomize number of orbiting bodies: 2–10
        let mut bodies = vec![Body::new(
            0,
            0.0,
            0.0,
            0.0,
            0.0,
            central_mass,
            [0.6, 0.0, 1.0],
        )];

        for i in 1..=n_orbiting {
            let r: f64 = rng.random_range(0.03..0.18); // random distance from center
            let angle: f64 = rng.random_range(0.0..std::f64::consts::TAU); // random position on that orbit ring
            let x = r * angle.cos(); // polar → cartesian
            let y = r * angle.sin();

            let v = (G * central_mass / r).sqrt(); // same orbital formula, but M and r vary

            let direction: f64 = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
            let vx = -direction * v * angle.sin(); // tangent to the orbit at this angle
            let vy = direction * v * angle.cos();

            let vx = vx + rng.random_range(-5.0..5.0); // Small random perturbation so orbits aren't perfectly stable
            let vy = vy + rng.random_range(-5.0..5.0);

            let mass: f64 = rng.random_range(1e11..5e12); // // Random small mass: 1×10¹¹ → 5×10¹²

            let color = [
                rng.random_range(0.4..1.0_f32),
                rng.random_range(0.4..1.0_f32),
                rng.random_range(0.4..1.0_f32),
            ];
            bodies.push(Body::new(i, x, y, vx, vy, mass, color));
        }
        Self { bodies }
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
