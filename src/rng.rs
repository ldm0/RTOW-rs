use lazy_static::lazy_static;
use rand;
use rand::SeedableRng;
use std::sync::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<rand::rngs::SmallRng> =
        Mutex::new(rand::rngs::SmallRng::from_entropy());
}
