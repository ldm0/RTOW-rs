use crate::ray;
use crate::rng;
use crate::vec;
use rand::Rng;

fn random_in_unit_disk() -> vec::Vec3 {
    let mut rng = rng::RNG.lock().unwrap();
    let degree = rng.gen_range(0., 2. * std::f32::consts::PI);
    vec::Vec3::new(degree.sin(), degree.cos(), 0.)
}

pub struct Camera {
    upper_left_corner: vec::Vec3,
    horizontal: vec::Vec3,
    vertical: vec::Vec3,
    origin: vec::Vec3,
    // u and v are all unified
    u: vec::Vec3,
    v: vec::Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        origin: vec::Vec3,
        lookat: vec::Vec3,
        upwards: vec::Vec3,
        v_aspect: f32,
        aspect_ratio: f32,
        aperture: f32,
    ) -> Self {
        let focus_distance = (lookat - origin).length();

        let v = v_aspect * std::f32::consts::PI / 180. / 2.;

        let half_height = v.tan();
        let half_width = aspect_ratio * half_height;

        let w = (origin - lookat).unify();
        let u = upwards.cross(w).unify();
        // Since u and v are unified, we can ensure v is unified too. (|v| = |u| x |w| * sin(90))
        let v = u.cross(w);
        println!(
            "u:{:?} v:{:?} w:{:?} half_width:{} half_height:{}",
            u, v, w, half_width, half_height
        );

        let (half_vertical, half_horizontal) = (half_height * v, half_width * u);
        let upper_left_corner = origin - focus_distance * (w + half_vertical + half_horizontal);
        println!(
            "half_horizontal:{:?} half_vertical:{:?}",
            half_horizontal, half_vertical
        );
        Self {
            upper_left_corner,
            horizontal: 2. * focus_distance * half_horizontal,
            vertical: 2. * focus_distance * half_vertical,
            origin,
            u,
            v,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> ray::Ray {
        let disk = self.lens_radius * random_in_unit_disk();
        let offset = disk.x * self.u + disk.y * self.v;
        let origin = self.origin + offset;
        let direction = self.upper_left_corner + u * self.horizontal + v * self.vertical - origin;
        ray::Ray::new(origin, direction)
    }
}
