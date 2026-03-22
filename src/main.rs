mod body;
mod renderer;
mod sim;

use renderer::Renderer;

fn main() {
    let mut sim = sim::Simulation::new();
    let mut renderer = Renderer::new();

    while renderer.is_open() {
        for _ in 0..10 {
            // multiple sim steps per frame
            sim.step(0.0001);
        }
        renderer.draw(&sim.bodies);
    }
}
