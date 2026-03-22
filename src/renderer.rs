use crate::body::Body;
use minifb::{Key, Window, WindowOptions};

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 800;

// How much of world-space to show. Bodies orbit at ~0.07 units so show ±0.15
const VIEW_SCALE: f64 = 0.15;

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
        // Clear to black
        self.buffer.fill(0);

        for body in bodies {
            let (sx, sy) = world_to_screen(body.pos[0], body.pos[1]);
            draw_circle(&mut self.buffer, sx, sy, 4, 0x00_FF_FF_FF);
        }

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn world_to_screen(x: f64, y: f64) -> (i32, i32) {
    let sx = ((x + VIEW_SCALE) / (2.0 * VIEW_SCALE) * WIDTH as f64) as i32;
    let sy = ((-y + VIEW_SCALE) / (2.0 * VIEW_SCALE) * HEIGHT as f64) as i32; // flip Y
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
