mod body;
mod quadtree;
mod renderer;
mod sim;

use renderer::Renderer;

fn main() {
    let mut sim = sim::Simulation::new();
    let mut renderer = Renderer::new();

    const DT: f64 = 0.0000001; // time step
    const SUBSTEPS: usize = 50; // calculations per frame

    while renderer.is_open() {
        for _ in 0..SUBSTEPS {
            sim.step(DT);
        }
        renderer.draw(&sim.bodies);
    }
}
