use crate::vec;

#[derive(Debug)]
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
}

impl Ray {
    pub fn new(origin: vec::Vec3, direction: vec::Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> vec::Vec3 {
        self.origin + t * self.direction
    }
}
