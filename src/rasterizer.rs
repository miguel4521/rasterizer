use na::{Matrix4, Vector2, Vector3};

pub struct Rasterizer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
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