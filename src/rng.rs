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
    let mut rng = RNG.lock().unwrap();
    let mut result;
    loop {
        result = vec::Vec3::new(
            rng.gen_range(-1., 1.),
            rng.gen_range(-1., 1.),
            rng.gen_range(-1., 1.),
        );
        if result.length() < 1. {
            break;
        }
    }
    result
}

/// `normal` is just used for direction checking, unifying is not needed
pub fn random_in_hemisphere(normal: vec::Vec3) -> vec::Vec3 {
    let tmp = random_in_unit_sphere();
    if tmp.dot(normal) >= 0. {
        tmp
    } else {
        -tmp
    }
}

pub fn random_in_unit_disk() -> vec::Vec3 {
    let mut rng = RNG.lock().unwrap();
    let mut x;
    let mut y;
    loop {
        x = rng.gen_range(0., 1.);
        y = rng.gen_range(0., 1.);
        if x * x + y * y < 1. {
            break;
        }
    }

    vec::Vec3::new(x, y, 0.)
}

pub fn random_color() -> vec::Vec3 {
    let mut rng = RNG.lock().unwrap();
    vec::Vec3::new(
        rng.gen_range(0., 1.),
        rng.gen_range(0., 1.),
        rng.gen_range(0., 1.),
    )
}
