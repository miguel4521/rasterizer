use crate::{
    model,
    vecs::{vec2::Vec2f, vec3::Vec3f},
};

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

                let x0 = ((v0.x + 1.0) * self.width as f32 / 2.0) as i32;
                let y0 = (self.height as f32 - ((v0.y + 1.0) * self.height as f32 / 2.0)) as i32;
                let x1 = ((v1.x + 1.0) * self.width as f32 / 2.0) as i32;
                let y1 = (self.height as f32 - ((v1.y + 1.0) * self.height as f32 / 2.0)) as i32;

                self.draw_line(&Vec2f::new(x0, y0), &Vec2f::new(x1, y1), &0xFFFFFF);
            }
        }
    }

    pub fn draw_triangle(&mut self, v0: &Vec2f, v1: &Vec2f, v2: &Vec2f, color: &u32) {
        self.draw_line(v0, v1, color);
        self.draw_line(v1, v2, color);
        self.draw_line(v2, v0, color);
    }

    pub fn fill_triangle(&mut self, t0: &Vec2f, t1: &Vec2f, t2: &Vec2f, color: &u32) {
        let pts = [t0, t1, t2];
        let bboxmin = Vec2f::new(
            pts.iter().map(|v| v.x).min().unwrap(),
            pts.iter().map(|v| v.y).min().unwrap(),
        );
        let bboxmax = Vec2f::new(
            pts.iter().map(|v| v.x).max().unwrap(),
            pts.iter().map(|v| v.y).max().unwrap(),
        );

        for x in bboxmin.x..=bboxmax.x {
            for y in bboxmin.y..=bboxmax.y {
                let p = Vec2f::new(x, y);
                let bc_screen = barycentric(&pts, &p);
                if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                    continue;
                }

                if p.x >= 0 && p.x < self.width as i32 && p.y >= 0 && p.y < self.height as i32 {
                    let index = (p.x + p.y * self.width as i32) as usize;
                    self.buffer[index] = *color;
                }
            }
        }
    }
}

fn barycentric(pts: &[&Vec2f; 3], p: &Vec2f) -> Vec3f {
    let u = Vec3f::new(
        (pts[2].x - pts[0].x) as f32,
        (pts[1].x - pts[0].x) as f32,
        (pts[0].x - p.x) as f32,
    )
    .cross(&Vec3f::new(
        (pts[2].y - pts[0].y) as f32,
        (pts[1].y - pts[0].y) as f32,
        (pts[0].y - p.y) as f32,
    ));

    // `pts` and `p` have integer values as coordinates
    // so `abs(u[2])` < 1 means `u[2]` is 0, which means
    // the triangle is degenerate
    if u.z.abs() < 1.0 {
        return Vec3f::new(-1.0, 1.0, 1.0);
    }
    Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
}
