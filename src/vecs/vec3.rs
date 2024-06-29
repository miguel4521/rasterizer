#[derive(Debug, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn subtract(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
