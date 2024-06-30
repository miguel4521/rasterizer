use crate::{
    model,
    vecs::{
        vec2::{self, Vec2f, Vec2i},
        vec3::Vec3f,
    },
};

pub struct Rasterizer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    light_dir: Vec3f,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Rasterizer {
        let mut zbuffer = vec![std::f32::MIN; width * height];
        for depth in zbuffer.iter_mut() {
            *depth = std::f32::MIN;
        }

        Rasterizer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer,
            light_dir: Vec3f::new(0.0, 0.0, -1.0), // Default light direction
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = std::f32::MIN;
        }
    }

    pub fn draw_line(&mut self, vec0: &Vec2i, vec1: &Vec2i, color: &u32) {
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
            let mut pts = [Vec3f::new(0.0, 0.0, 0.0); 3];

            for j in 0..3 {
                let v = model.vert(face[j]);
                pts[j] = self.world2screen(v);
            }

            let mut world_coords = [Vec3f::new(0.0, 0.0, 0.0); 3];
            for j in 0..3 {
                world_coords[j] = model.vert(face[j]);
            }

            let mut n = (world_coords[2] - world_coords[0])
                .cross(&(world_coords[1] - world_coords[0]));
            n.normalize();

            let intensity = n.dot(&self.light_dir);
            if intensity > 0.0 {
                let color = ((intensity * 255.0) as u32) << 16
                    | ((intensity * 255.0) as u32) << 8
                    | (intensity * 255.0) as u32;
                self.draw_triangle(pts, color);
            }
        }
    }

    pub fn draw_triangle(&mut self, pts: [Vec3f; 3], color: u32) {
        let mut bboxmin = Vec2f::new(std::f32::MAX, std::f32::MAX);
        let mut bboxmax = Vec2f::new(std::f32::MIN, std::f32::MIN);
        let clamp = Vec2f::new((self.width - 1) as f32, (self.height - 1) as f32);

        // Calculate bounding box
        for i in 0..3 {
            for j in 0..2 {
                bboxmin[j] = (0.0 as f32).max(bboxmin[j].min(pts[i][j]));
                bboxmax[j] = clamp[j].min(bboxmax[j].max(pts[i][j]));
            }
        }

        let mut p = Vec3f::new(0.0, 0.0, 0.0);
        for x in bboxmin.x as i32..=bboxmax.x as i32 {
            for y in bboxmin.y as i32..=bboxmax.y as i32 {
                p.x = x as f32;
                p.y = y as f32;
                let bc_screen = barycentric(pts, &p);
                if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                    continue;
                }

                p.z = 0.0;
                for i in 0..3 {
                    p.z += pts[i].z * bc_screen[i];
                }

                let idx = (x + y * self.width as i32) as usize;
                if self.zbuffer[idx] < p.z {
                    self.zbuffer[idx] = p.z;
                    self.buffer[idx] = color;
                }
            }
        }
    }

    fn world2screen(&self, v: Vec3f) -> Vec3f {
        Vec3f::new(
            ((v.x + 1.0) * self.width as f32 / 2.0).round(),
            ((-v.y + 1.0) * self.height as f32 / 2.0).round(),
            v.z,
        )
    }
}

fn barycentric(pts: [Vec3f; 3], p: &Vec3f) -> Vec3f {
    let mut s = [Vec3f::new(0.0, 0.0, 0.0); 2];

    for i in 0..2 {
        s[i].x = pts[2][i] - pts[0][i];
        s[i].y = pts[1][i] - pts[0][i];
        s[i].z = pts[0][i] - p[i];
    }

    let u = s[0].cross(&s[1]);

    if u.z.abs() > 1e-2 {
        return Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    }

    Vec3f::new(-1.0, 1.0, 1.0)
}