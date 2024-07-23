use na::{Matrix4, Point3, Unit, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera {
            position,
            target,
            up,
            fov,
            aspect_ratio,
            near,
            far,
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &self.target, &self.up)
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect_ratio, self.fov, self.near, self.far)
    }

    pub fn move_forward(&mut self, amount: f32) {
        let direction = (self.target - self.position).normalize();
        self.position += direction * amount;
        self.target += direction * amount;
    }

    pub fn move_backward(&mut self, amount: f32) {
        self.move_forward(-amount);
    }

    pub fn move_left(&mut self, amount: f32) {
        let direction = (self.target - self.position).normalize();
        let left = self.up.cross(&direction).normalize();
        self.position += left * amount;
        self.target += left * amount;
    }

    pub fn move_right(&mut self, amount: f32) {
        self.move_left(-amount);
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        let direction = (self.target - self.position).normalize();
        let rotation = Matrix4::from_axis_angle(&Unit::new_normalize(self.up), angle);
        let new_direction = rotation.transform_vector(&direction);
        self.target = self.position + new_direction;
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        let direction = (self.target - self.position).normalize();
        let right = self.up.cross(&direction).normalize();
        let rotation = Matrix4::from_axis_angle(&Unit::new_normalize(right), angle);
        let new_direction = rotation.transform_vector(&direction);
        self.target = self.position + new_direction;
    }
}
