use crate::vec;
use lazy_static::lazy_static;
use rand;
use rand::{Rng, SeedableRng};
use std::sync::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<rand::rngs::SmallRng> =
        Mutex::new(rand::rngs::SmallRng::from_entropy());
}

pub fn random_unit() -> f32 {
    let mut rng = RNG.lock().unwrap();
    rng.gen_range(0., 1.)
}

pub fn random_in_unit_sphere() -> vec::Vec3 {
    // This is true random, the one in RTOW is not uniform
    let mut rng = RNG.lock().unwrap();
    let a = rng.gen_range(0., 2. * std::f32::consts::PI);
    let b = rng.gen_range(-1., 1. * std::f32::consts::PI);
    let (sin_a, sin_b) = (a.sin(), b.sin());
    let (cos_a, cos_b) = (a.cos(), b.cos());
    vec::Vec3::new(sin_a * cos_b, sin_a * sin_b, cos_a)
}

/// `normal` is just used for direction checking, is not needed to be unified
pub fn random_in_hemisphere(normal: vec::Vec3) -> vec::Vec3 {
    let tmp = random_in_unit_sphere();
    if tmp.dot(normal) >= 0. {
        tmp
    } else {
        -tmp
    }
}

pub fn random_color() -> vec::Vec3 {
    vec::Vec3::new(random_unit(), random_unit(), random_unit())
}
