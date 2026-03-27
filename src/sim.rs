use crate::body::Body;
use crate::quadtree::{BoundingBox, QuadTree};
use rand::RngExt;

const G: f64 = 6.674e-11;
const SOFTENING: f64 = 5e-3; // prevent infinite force when bodies overlap
const THETA: f64 = 0.5;

pub struct Simulation {
    pub bodies: Vec<Body>,
    forces: Vec<[f64; 2]>, // carried between steps
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
        let mut sim = Self {
            bodies,
            forces: vec![[0.0; 2]; n_orbiting + 1],
        };
        sim.forces = sim.build_and_compute_forces();
        sim
    }

    pub fn step(&mut self, dt: f64) {
        // Half kick - old forces
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.vel[0] += self.forces[i][0] * dt / 2.0;
            body.vel[1] += self.forces[i][1] * dt / 2.0;
        }
        // Drift
        for body in self.bodies.iter_mut() {
            body.pos[0] += body.vel[0] * dt;
            body.pos[1] += body.vel[1] * dt;
        }
        self.forces = self.build_and_compute_forces();
        // Half kick - new forces
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.vel[0] += self.forces[i][0] * dt / 2.0;
            body.vel[1] += self.forces[i][1] * dt / 2.0;
        }
        // After the integration loop - subtract mean velocity to keep system in CoM frame
        let total_mass: f64 = self.bodies.iter().map(|b| b.mass).sum();
        let mean_vx = self.bodies.iter().map(|b| b.vel[0] * b.mass).sum::<f64>() / total_mass;
        let mean_vy = self.bodies.iter().map(|b| b.vel[1] * b.mass).sum::<f64>() / total_mass;
        for body in self.bodies.iter_mut() {
            body.vel[0] -= mean_vx;
            body.vel[1] -= mean_vy;
        }
    }

    fn build_and_compute_forces(&self) -> Vec<[f64; 2]> {
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
        let half = ((max_x - min_x).max(max_y - min_y) / 2.0).max(0.1);

        let bounds = BoundingBox { cx, cy, half };
        let mut tree = QuadTree::Empty(bounds);
        for body in &self.bodies {
            tree.insert(body);
        }

        let softening_sq = SOFTENING * SOFTENING;
        let theta_sq = THETA * THETA;

        use rayon::prelude::*;
        let mut forces = vec![[0.0_f64; 2]; self.bodies.len()];
        forces
            .par_iter_mut()
            .zip(self.bodies.par_iter())
            .for_each(|(force, body)| {
                *force = tree.compute_force(body, theta_sq, G, softening_sq);
            });
        forces
    }
}

/*
Barnes-Hut KDK Leapfrog Integration:

build quadtree from current positions
→ compute accelerations via θ criterion (O(n log n))

half kick:  vel += accel * dt/2        (old forces)
drift:      pos += vel * dt
rebuild quadtree at new positions
→ recompute accelerations

half kick:  vel += accel * dt/2        (new forces)

subtract mass-weighted mean velocity   (CoM frame correction)
*/

