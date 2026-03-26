mod body;
mod quadtree;
mod renderer;
mod sim;

use renderer::Renderer;

fn main() {
    let mut sim = sim::Simulation::new();
    let mut renderer = Renderer::new();

    while renderer.is_open() {
        for _ in 0..200 {
            sim.step(0.0000001); // 10x smaller dt, 20x more steps
        }
        renderer.draw(&sim.bodies);
    }
}
