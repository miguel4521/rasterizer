use crate::model;

pub struct Window {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: minifb::Window,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        let buffer = vec![0; width * height];
        let window = minifb::Window::new(
            "Minimal Renderer - ESC to exit",
            width,
            height,
            minifb::WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Window {
            width,
            height,
            buffer,
            window,
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn clear(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0; // Black
        }
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x0;
        let mut y = y0;

        while x != x1 || y != y1 {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let index = (x + y * self.width as i32) as usize;
                self.buffer[index] = color;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn render_wireframe(&mut self, model: &model::Model) {
        for i in 0..model.nfaces() {
            let face = model.face(i);

            for j in 0..3 {
                let v0 = model.vert(face[j]);
                let v1 = model.vert(face[(j + 1) % 3]);

                let x0 = ((v0.raw[0] + 1.0) * self.width as f32 / 2.0) as i32;
                let y0 =
                    (self.height as f32 - ((v0.raw[1] + 1.0) * self.height as f32 / 2.0)) as i32;
                let x1 = ((v1.raw[0] + 1.0) * self.width as f32 / 2.0) as i32;
                let y1 =
                    (self.height as f32 - ((v1.raw[1] + 1.0) * self.height as f32 / 2.0)) as i32;

                self.line(x0, y0, x1, y1, 0xFFFFFF);
            }
        }
    }
}
