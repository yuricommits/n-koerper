mod body;
mod quadtree;
mod renderer;
mod sim;

use renderer::Renderer;

fn main() {
    let mut sim = sim::Simulation::new();
    let mut renderer = Renderer::new();

    while renderer.is_open() {
        for _ in 0..100 {
            sim.step(0.000001); // 100 × smaller dt = same time, far more stable
        }
        renderer.draw(&sim.bodies);
    }
}
