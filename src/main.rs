mod body;
mod renderer;
mod sim;

fn main() {
    let mut sim = sim::Simulation::new();
    for _ in 0..100 {
        sim.step(0.0001);
        println!("{:?}", sim.bodies[0].pos);
    }
}
