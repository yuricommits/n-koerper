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

    fn draw_grid(&mut self, com_x: f64, com_y: f64, view_scale: f64) {
        // Pick a fixed spacing that gives 4–10 lines on screen
        // Snap to nearest power of 10 so labels stay clean
        let raw_spacing = view_scale / 5.0;
        let magnitude = raw_spacing.log10().floor();
        let grid_spacing = 10f64.powf(magnitude); // snaps to 0.001, 0.01, 0.1, 1.0, etc.

        let axis_color = 0x00_55_55_55u32;
        let grid_color = 0x00_22_22_22u32;

        // find first grid line left of screen, step right
        let start_x = ((-(view_scale)) / grid_spacing).floor() as i32;
        let end_x = ((view_scale) / grid_spacing).ceil() as i32;

        for i in start_x..=end_x {
            let world_x = i as f64 * grid_spacing;
            let (sx, _) = world_to_screen(world_x - com_x, 0.0, view_scale);
            let color = if i == 0 { axis_color } else { grid_color };
            for y in 0..HEIGHT as i32 {
                self.draw_pixel(sx, y, color);
            }
        }

        let start_y = (-(view_scale) / grid_spacing).floor() as i32; // negative first
        let end_y = ((view_scale) / grid_spacing).ceil() as i32; // positive last

        for i in start_y..=end_y {
            let world_y = i as f64 * grid_spacing;
            let (_, sy) = world_to_screen(0.0, world_y - com_y, view_scale); // ← (_, sy)
            let color = if i == 0 { axis_color } else { grid_color };
            for x in 0..WIDTH as i32 {
                self.draw_pixel(x, sy, color); // ← draw_pixel(x, sy)
            }
        }
    }

    fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && y >= 0 && (x as usize) < WIDTH && (y as usize) < HEIGHT {
            self.buffer[y as usize * WIDTH + x as usize] = color;
        }
    }

    pub fn draw(&mut self, bodies: &[Body]) {
        self.buffer.fill(0);

        let total_mass: f64 = bodies.iter().map(|b| b.mass).sum();
        let com_x: f64 = bodies.iter().map(|b| b.pos[0] * b.mass).sum::<f64>() / total_mass;
        let com_y: f64 = bodies.iter().map(|b| b.pos[1] * b.mass).sum::<f64>() / total_mass;

        let mut dists: Vec<f64> = bodies
            .iter()
            .map(|b| {
                let dx = b.pos[0] - com_x;
                let dy = b.pos[1] - com_y;
                (dx * dx + dy * dy).sqrt()
            })
            .collect();
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p90_index = (dists.len() as f64 * 0.9) as usize;
        let view_scale = (dists[p90_index] * 1.5).max(0.1);

        self.draw_grid(com_x, com_y, view_scale);

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
