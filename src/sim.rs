use crate::body::Body;
use crate::quadtree::{BoundingBox, QuadTree};
use rand::RngExt;

const G: f64 = 6.674e-11;
const SOFTENING: f64 = 5e-3; // prevent infinite force when bodies overlap
const THETA: f64 = 0.5;

pub struct Simulation {
    pub bodies: Vec<Body>,
}

impl Simulation {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let central_mass: f64 = rng.random_range(1e12..1e13); // Randomize the central heavy mass: 1×10¹² → 1×10¹³
        let n_orbiting: usize = rng.random_range(100..=500); // Randomize number of orbiting bodies: 100–500
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
            let r: f64 = rng.random_range(0.05..0.5); // random distance from center
            let angle: f64 = rng.random_range(0.0..std::f64::consts::TAU); // random position on that orbit ring
            let x = r * angle.cos(); // polar → cartesian
            let y = r * angle.sin();

            let v = (G * central_mass / r).sqrt(); // same orbital formula, but M and r vary

            // let direction: f64 = if rng.random_bool(0.5) { 1.0 } else { -1.0 }; // 50/50
            let direction: f64 = 1.0; // CCW
            let vx = -direction * v * angle.sin(); // tangent to the orbit at this angle
            let vy = direction * v * angle.cos();

            let vx = vx + rng.random_range(-5.0..5.0); // Small random perturbation so orbits aren't perfectly stable
            let vy = vy + rng.random_range(-5.0..5.0);

            let mass: f64 = rng.random_range(1e11..5e12); // Random small mass: 1×10¹¹ → 5×10¹²

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

        let min_x = self
            .bodies
            .iter()
            .map(|b| b.pos[0])
            .fold(f64::INFINITY, f64::min);
        let max_x = self
            .bodies
            .iter()
            .map(|b| b.pos[0])
            .fold(f64::NEG_INFINITY, f64::max);
        let min_y = self
            .bodies
            .iter()
            .map(|b| b.pos[1])
            .fold(f64::INFINITY, f64::min);
        let max_y = self
            .bodies
            .iter()
            .map(|b| b.pos[1])
            .fold(f64::NEG_INFINITY, f64::max);

        let cx = (min_x + max_x) / 2.0;
        let cy = (min_y + max_y) / 2.0;
        let half = ((max_x - min_x).max(max_y - min_y) / 2.0).max(0.1); // square + minimum size guard

        let bounds = BoundingBox { cx, cy, half };

        // Build quadtree
        let mut tree = QuadTree::Empty(bounds);
        for body in &self.bodies {
            tree.insert(body);
        }

        // Precompute constants once per timestep
        let softening_sq = SOFTENING * SOFTENING;
        let theta_sq = THETA * THETA;

        for (i, body) in self.bodies.iter().enumerate() {
            forces[i] = tree.compute_force(body, theta_sq, G, softening_sq);
        }

        let accels = forces; // rename mentally — these are already accelerations

        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.vel[0] += accels[i][0] * dt; // no / body.mass
            body.vel[1] += accels[i][1] * dt;
            body.pos[0] += body.vel[0] * dt;
            body.pos[1] += body.vel[1] * dt;
        }

        // After the integration loop — subtract mean velocity to keep system in CoM frame
        let total_mass: f64 = self.bodies.iter().map(|b| b.mass).sum();
        let mean_vx: f64 = self.bodies.iter().map(|b| b.vel[0] * b.mass).sum::<f64>() / total_mass;
        let mean_vy: f64 = self.bodies.iter().map(|b| b.vel[1] * b.mass).sum::<f64>() / total_mass;
        for body in self.bodies.iter_mut() {
            body.vel[0] -= mean_vx;
            body.vel[1] -= mean_vy;
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
