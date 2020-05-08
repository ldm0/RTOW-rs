use rand::Rng;

use crate::hit;
use crate::rng::RNG;
use crate::vec;

/// We assume normal is always at the same side with the in_direction. Normal and
/// in_direction are not unified.
fn reflect(in_direction: vec::Vec3, normal: vec::Vec3) -> vec::Vec3 {
    let normal = normal.unify();
    in_direction - 2. * in_direction.dot(normal) * normal
}

/// We assume normal is always at the same side with the in_direction. Normal and
/// in_direction are not unified.
fn refract(coefficient: f32, in_direction: vec::Vec3, normal: vec::Vec3) -> vec::Vec3 {
    let (r, n) = (in_direction.unify(), normal.unify());
    let cos_theta = r.dot(n);
    if coefficient > 1. && (1. - cos_theta * cos_theta) * coefficient * coefficient > 1. {
        // All reflect
        reflect(in_direction, normal)
    } else {
        // refract
        let r_perpendicular = coefficient *  (r - cos_theta * n);
        let r_parallel = -(1. - r_perpendicular.length_squared()).sqrt() * n;
        r_parallel + r_perpendicular
    }
}

fn random_in_unit_sphere() -> vec::Vec3 {
    // This is true random, the one in RTOW is not uniform
    let mut rng = RNG.lock().unwrap();
    let a = rng.gen_range(0., 2. * std::f32::consts::PI);
    let b = rng.gen_range(-1., 1. * std::f32::consts::PI);
    let (sin_a, sin_b) = (a.sin(), b.sin());
    let (cos_a, cos_b) = (a.cos(), b.cos());
    vec::Vec3::new(sin_a * cos_b, sin_a * sin_b, cos_a)
}

/// `normal` is just used for direction checking, is not needed to be unified
fn random_in_hemisphere(normal: vec::Vec3) -> vec::Vec3 {
    let tmp = random_in_unit_sphere();
    if tmp.dot(normal) >= 0. {
        tmp
    } else {
        -tmp
    }
}

pub trait Material {
    /// Given ray_in direction and normal return attenuation and scattered
    /// direction. When reflected direction is illegal(e.g. For in-transparent
    /// material, reflected dot normal is positive), return None.
    fn scatter(
        &self,
        in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)>;
}

pub struct Metal {
    albedo: vec::Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(fuzz: f32, albedo: vec::Vec3) -> Self {
        if fuzz < 0. && fuzz >= 1. {
            panic!("invalid fuzz: not in [0, 1)")
        }
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        let reflected = reflect(in_direction, hit_record.normal) + self.fuzz * random_in_unit_sphere(); // fuzz a little bit
        Some((self.albedo, reflected))
    }
}

pub struct Lambertian {
    albedo: vec::Vec3,
}

impl Lambertian {
    pub fn new(albedo: vec::Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        Some((self.albedo, random_in_hemisphere(hit_record.normal)))
    }
}

pub struct Glass {
    eta: f32,
}

impl Glass {
    pub fn new(eta: f32) -> Self {
        Self { eta }
    }
}

impl Material for Glass {
    fn scatter(
        &self,
        in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        // We assume two glass bell never overlaps, so always 1 / eta and eta /
        // 1 when in and out.
        let coefficient = if hit_record.front_face {
            // in
            1. / self.eta
        } else {
            // out
            self.eta
        };
        let refracted = refract(coefficient, in_direction, hit_record.normal);
        Some((vec::Vec3::new(1., 1., 1.), refracted))
    }
}
