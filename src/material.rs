use rand::Rng;

use crate::rng::RNG;
use crate::vec;

fn reflect(in_direction: vec::Vec3, normal: vec::Vec3) -> Option<vec::Vec3> {
    let normal = normal.unify();
    let reflect = in_direction - 2. * in_direction.dot(normal) * normal;
    if reflect.dot(normal) >= 0. {
        Some(reflect)
    } else {
        None
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
    fn scatter(&self, in_direction: vec::Vec3, normal: vec::Vec3)
        -> Option<(vec::Vec3, vec::Vec3)>;
}

pub struct Metal {
    albedo: vec::Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(fuzz: f32, albedo: vec::Vec3) -> Self {
        if fuzz < 0. && fuzz >= 1.{
            panic!("invalid fuzz: not in [0, 1)")
        }
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        in_direction: vec::Vec3,
        normal: vec::Vec3,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        reflect(in_direction, normal)
        .map(|reflected| reflected + self.fuzz * random_in_unit_sphere())  // fuzz a little bit
        .map(|reflected| (self.albedo, reflected))
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
        normal: vec::Vec3,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        Some((self.albedo, random_in_hemisphere(normal)))
    }
}

pub struct Glass {
    
}
