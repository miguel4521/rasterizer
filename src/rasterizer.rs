use na::{Matrix4, Vector2, Vector3};

use crate::model;

pub struct Rasterizer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    light_dir: Vector3<f32>,
    texture: Option<Vec<u32>>,
    texture_width: usize,
    texture_height: usize,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
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
            light_dir: Vector3::new(0.0, 0.0, -1.0), // Default light direction
            texture: None,
            texture_width: 0,
            texture_height: 0,
            view_matrix: Matrix4::identity(),
            projection_matrix: Matrix4::identity(),
        }
    }

    pub fn set_view_projection(
        &mut self,
        view_matrix: Matrix4<f32>,
        projection_matrix: Matrix4<f32>,
    ) {
        self.view_matrix = view_matrix;
        self.projection_matrix = projection_matrix;
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = std::f32::MIN;
        }
    }

    pub fn draw_line(&mut self, vec0: &Vector2<i32>, vec1: &Vector2<i32>, color: &u32) {
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
        let mvp = self.projection_matrix * self.view_matrix;

        for i in 0..model.nfaces() {
            let face = model.face(i);
            let mut pts = [Vector3::new(0.0, 0.0, 0.0); 3];
            let mut tex_coords = [Vector2::new(0.0, 0.0); 3];
            let mut world_coords = [Vector3::new(0.0, 0.0, 0.0); 3];

            for j in 0..3 {
                let v = model.vert(face[j].0);
                let v_homogeneous = v.insert_row(3, 1.0); // Convert to homogeneous coordinates
                let screen_v = mvp * v_homogeneous; // Apply MVP transformation
                pts[j] = self.world2screen(screen_v.xyz()); // Convert to screen space
                tex_coords[j] = model.texcoord(face[j].1);
                world_coords[j] = v;
            }

            let n = (world_coords[2] - world_coords[0])
                .cross(&(world_coords[1] - world_coords[0]))
                .normalize();

            let intensity = n.dot(&self.light_dir);
            if intensity > 0.0 {
                let color = ((intensity * 255.0) as u32) << 16
                    | ((intensity * 255.0) as u32) << 8
                    | (intensity * 255.0) as u32;
                self.draw_triangle(pts, tex_coords, color);
            }
        }
    }

    pub fn draw_triangle(
        &mut self,
        pts: [Vector3<f32>; 3],
        tex_coords: [Vector2<f32>; 3],
        color: u32,
    ) {
        let mut bboxmin = Vector2::new(std::f32::MAX, std::f32::MAX);
        let mut bboxmax = Vector2::new(std::f32::MIN, std::f32::MIN);
        let clamp = Vector2::new((self.width - 1) as f32, (self.height - 1) as f32);

        // Calculate bounding box
        for i in 0..3 {
            for j in 0..2 {
                bboxmin[j] = (0.0 as f32).max(bboxmin[j].min(pts[i][j]));
                bboxmax[j] = clamp[j].min(bboxmax[j].max(pts[i][j]));
            }
        }

        let mut p = Vector3::new(0.0, 0.0, 0.0);
        for x in bboxmin.x as i32..=bboxmax.x as i32 {
            for y in bboxmin.y as i32..=bboxmax.y as i32 {
                p.x = x as f32;
                p.y = y as f32;
                let bc_screen = barycentric(pts, &p);
                if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                    continue;
                }

                p.z = 0.0;
                let mut tex_u = 0.0;
                let mut tex_v = 0.0;
                for i in 0..3 {
                    p.z += pts[i].z * bc_screen[i];
                    tex_u += tex_coords[i].x * bc_screen[i];
                    tex_v += tex_coords[i].y * bc_screen[i];
                }

                let idx = (x + y * self.width as i32) as usize;
                if self.zbuffer[idx] < p.z {
                    self.zbuffer[idx] = p.z;
                    if let Some(ref texture) = self.texture {
                        let tex_x = (tex_u * self.texture_width as f32) as usize;
                        let tex_y = ((1.0 - tex_v) * self.texture_height as f32) as usize;
                        let tex_idx = tex_x + tex_y * self.texture_width;
                        self.buffer[idx] = texture[tex_idx];
                    } else {
                        self.buffer[idx] = color;
                    }
                }
            }
        }
    }

    fn world2screen(&self, v: Vector3<f32>) -> Vector3<f32> {
        Vector3::new(
            ((v.x + 1.0) * self.width as f32 / 2.0).round(),
            ((-v.y + 1.0) * self.height as f32 / 2.0).round(),
            v.z,
        )
    }

    pub fn set_texture(&mut self, texture: Option<&Vec<u32>>, width: usize, height: usize) {
        self.texture = texture.cloned();
        self.texture_width = width;
        self.texture_height = height;
    }
}

fn barycentric(pts: [Vector3<f32>; 3], p: &Vector3<f32>) -> Vector3<f32> {
    let mut s = [Vector3::new(0.0, 0.0, 0.0); 2];

    for i in 0..2 {
        s[i].x = pts[2][i] - pts[0][i];
        s[i].y = pts[1][i] - pts[0][i];
        s[i].z = pts[0][i] - p[i];
    }

    let u = s[0].cross(&s[1]);

    if u.z.abs() > 1e-2 {
        return Vector3::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
    }

    Vector3::new(-1.0, 1.0, 1.0)
}
