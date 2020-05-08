use crate::ray;
use crate::vec;

pub struct Camera {
    upper_left_corner: vec::Vec3,
    horizontal: vec::Vec3,
    vertical: vec::Vec3,
    origin: vec::Vec3,
}

impl Camera {
    pub fn new(
        origin: vec::Vec3,
        lookat: vec::Vec3,
        upwards: vec::Vec3,
        v_aspect: f32,
        h_aspect: f32,
    ) -> Self {
        let v = v_aspect / 180. * std::f32::consts::PI;
        let h = h_aspect / 180. * std::f32::consts::PI;

        let (half_width, half_height) = (h.tan(), v.tan());

        let w = (origin - lookat).unify();
        let u = upwards.cross(w).unify();
        let v = u.cross(w).unify();
        println!("u:{:?} v:{:?} w:{:?} half_width:{} half_height:{}", u, v, w, half_width, half_height);

        let (half_vertical, half_horizontal) = (half_height * v, half_width * u);
        let upper_left_corner = origin - w - half_vertical - half_horizontal;
        println!("half_horizontal:{:?} half_vertical:{:?}", half_horizontal, half_vertical);
        Self {
            upper_left_corner,
            horizontal: 2. * half_horizontal,
            vertical: 2. * half_vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        let direction = self.upper_left_corner + u * self.horizontal + v * self.vertical;
        ray::Ray::new(self.origin, direction)
    }
}
