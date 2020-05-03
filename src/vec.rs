use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn pixel(self) -> [u8; 3] {
        let tmp = self * 255.9999;
        [tmp.x as u8, tmp.y as u8, tmp.z as u8]
    }

    pub fn unify(self) -> Vec3 {
        self / self.length()
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn dot(self, b: Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn cross(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, b: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, b: f32) -> Self::Output {
        Vec3 {
            x: self.x + b,
            y: self.y + b,
            z: self.z + b,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, b: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, b: f32) -> Self::Output {
        Vec3 {
            x: self.x * b,
            y: self.y * b,
            z: self.z * b,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Self::Output {
        Vec3 {
            x: self * b.x,
            y: self * b.y,
            z: self * b.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, b: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * b.x,
            y: self.y * b.y,
            z: self.z * b.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, b: f32) -> Self::Output {
        self * (1. / b)
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, b: Vec3) -> Self::Output {
        Vec3 {
            x: self / b.x,
            y: self / b.y,
            z: self / b.z,
        }
    }
}
