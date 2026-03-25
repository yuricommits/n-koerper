use crate::body::Body;
use minifb::{Key, Window, WindowOptions};

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;

pub struct Renderer {
    pub buffer: Vec<u32>,
    pub window: Window,
}

impl Renderer {
    pub fn new() -> Self {
        let window = Window::new("n-körper", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        Self {
            buffer: vec![0; WIDTH * HEIGHT],
            window,
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn draw(&mut self, bodies: &[Body]) {
        self.buffer.fill(0);

        let total_mass: f64 = bodies.iter().map(|b| b.mass).sum();
        let com_x: f64 = bodies.iter().map(|b| b.pos[0] * b.mass).sum::<f64>() / total_mass;
        let com_y: f64 = bodies.iter().map(|b| b.pos[1] * b.mass).sum::<f64>() / total_mass;

        // Auto-scale: fit the furthest body on screen with 20% padding
        let max_dist = bodies
            .iter()
            .map(|b| {
                let dx = b.pos[0] - com_x;
                let dy = b.pos[1] - com_y;
                (dx * dx + dy * dy).sqrt()
            })
            .fold(0.1_f64, f64::max); // minimum 0.1 so it doesn't zoom in too close

        let view_scale = max_dist * 1.2;

        for body in bodies {
            let (sx, sy) = world_to_screen(body.pos[0] - com_x, body.pos[1] - com_y, view_scale);

            let r = (body.color[0] * 255.0) as u32;
            let g = (body.color[1] * 255.0) as u32;
            let b = (body.color[2] * 255.0) as u32;
            let color = (r << 16) | (g << 8) | b;

            let radius = ((body.radius / (2.0 * view_scale)) * WIDTH as f64) as i32;
            let radius = radius.max(2);

            draw_circle(&mut self.buffer, sx, sy, radius, color);
        }

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn world_to_screen(x: f64, y: f64, view_scale: f64) -> (i32, i32) {
    let sx = ((x + view_scale) / (2.0 * view_scale) * WIDTH as f64) as i32;
    let sy = ((-y + view_scale) / (2.0 * view_scale) * HEIGHT as f64) as i32;
    (sx, sy)
}

fn draw_circle(buffer: &mut Vec<u32>, cx: i32, cy: i32, r: i32, color: u32) {
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                let x = cx + dx;
                let y = cy + dy;
                if x >= 0 && y >= 0 && (x as usize) < WIDTH && (y as usize) < HEIGHT {
                    buffer[y as usize * WIDTH + x as usize] = color;
                }
            }
        }
    }
}
