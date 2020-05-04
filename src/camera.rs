use crate::ray;
use crate::vec;

pub struct Camera {
    upper_left_corner: vec::Vec3,
    horizontal: vec::Vec3,
    vertical: vec::Vec3,
    origin: vec::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let upper_left_corner = vec::Vec3::new(-2., 1., -1.);
        let horizontal = vec::Vec3::new(4., 0., 0.);
        let vertical = vec::Vec3::new(0., -2., 0.);
        let origin = vec::Vec3::new(0., 0., 0.);
        Self {
            upper_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        let direction = self.upper_left_corner + u * self.horizontal + v * self.vertical;
        ray::Ray::new(self.origin, direction)
    }
}
