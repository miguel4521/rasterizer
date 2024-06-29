use crate::model;

pub struct Vec2f {
    pub x: i32,
    pub y: i32,
}

impl Vec2f {
    pub fn new(x: i32, y: i32) -> Vec2f {
        Vec2f { x, y }
    }
}

pub struct Rasterizer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Rasterizer {
        Rasterizer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn draw_line(&mut self, vec0: &Vec2f, vec1: &Vec2f, color: &u32) {
        let mut x0 = vec0.x;
        let mut y0 = vec0.y;
        let mut x1 = vec1.x;
        let mut y1 = vec1.y;

        let mut steep = false;

        // Swap x and y if the line is steep
        if (x0 - x1).abs() < (y0 - y1).abs() {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
            steep = true;
        }

        // Ensure x0 < x1 to simplify the loop
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y0;

        for x in x0..=x1 {
            let (px, py) = if steep { (y, x) } else { (x, y) };

            // Check bounds before accessing buffer
            if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                let index = (px + py * self.width as i32) as usize;
                self.buffer[index] = *color;
            }

            error2 += derror2;

            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }
    pub fn draw_wireframe(&mut self, model: &model::Model) {
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

                self.draw_line(&Vec2f::new(x0, y0), &Vec2f::new(x1, y1), &0xFFFFFF);
            }
        }
    }

    pub fn draw_triangle(&mut self, v0: &Vec2f, v1: &Vec2f, v2: &Vec2f, color: &u32) {
        self.draw_line(v0, v1, color);
        self.draw_line(v1, v2, color);
        self.draw_line(v2, v0, color);
    }

    pub fn fill_triangle(&mut self, t0: &Vec2f, t1: &Vec2f, v2: &Vec2f, color: &u32) {
        // Sort vertices by y-coordinate
        let mut v0 = t0;
        let mut v1 = t1;
        let mut v2 = v2;

        // Sorting vertices by y-coordinate
        if v0.y > v1.y {
            std::mem::swap(&mut v0, &mut v1);
        }
        if v0.y > v2.y {
            std::mem::swap(&mut v0, &mut v2);
        }
        if v1.y > v2.y {
            std::mem::swap(&mut v1, &mut v2);
        }

        let total_height = v2.y - v0.y;

        // Early return if the triangle has no height
        if total_height == 0 {
            return;
        }

        for i in 0..=total_height {
            let second_half = i > v1.y - v0.y || v1.y == v0.y;
            let segment_height = if second_half {
                v2.y - v1.y
            } else {
                v1.y - v0.y
            };

            let alpha = i as f32 / total_height as f32;
            let beta = if second_half {
                (i - (v1.y - v0.y)) as f32 / segment_height as f32
            } else {
                i as f32 / segment_height as f32
            };

            let mut a = Vec2f::new(
                (v0.x as f32 + (v2.x as f32 - v0.x as f32) * alpha) as i32,
                v0.y + i,
            );

            let mut b = if second_half {
                Vec2f::new(
                    (v1.x as f32 + (v2.x as f32 - v1.x as f32) * beta) as i32,
                    v1.y + (i - (v1.y - v0.y)),
                )
            } else {
                Vec2f::new(
                    (v0.x as f32 + (v1.x as f32 - v0.x as f32) * beta) as i32,
                    v0.y + i,
                )
            };

            if a.x > b.x {
                std::mem::swap(&mut a, &mut b);
            }

            for j in a.x..=b.x {
                let p = Vec2f::new(j, a.y);
                let index = (p.x + p.y * self.width as i32) as usize;
                self.buffer[index] = *color;
            }
        }
    }
}
