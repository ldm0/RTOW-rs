use image;
use lazy_static::lazy_static;
use rand;
use std::sync::Mutex;

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec;

use rand::{Rng, SeedableRng};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;
const SAMPLE: u32 = 500;
const DEPTH: u32 = 50;

lazy_static! {
    static ref RNG: Mutex<rand::rngs::SmallRng> = Mutex::new(rand::rngs::SmallRng::from_entropy());
}

fn random_in_unit_sphere() -> vec::Vec3 {
    // This is true random, the one in RTOW is not uniform
    let mut rng = RNG.lock().unwrap();
    let a = rng.gen_range(0., 2. * std::f32::consts::PI);
    let b = rng.gen_range(0., 2. * std::f32::consts::PI);
    let (sin_a, sin_b) = (a.sin(), b.sin());
    let (cos_a, cos_b) = (a.cos(), b.cos());
    vec::Vec3::new(sin_a * cos_b, sin_a * sin_b, cos_a)
}

fn ray_color(ray: &ray::Ray, world: &hit::HittableList, depth: u32) -> vec::Vec3 {
    if depth == 0 {
        vec::Vec3::new(0., 0., 0.)
    } else if let Some(hit_record) = world.hit(&ray, 0., std::f32::MAX) {
        let uni_normal = hit_record.normal.unify();
        let reflect_ray = ray::Ray::new(hit_record.position, uni_normal + random_in_unit_sphere());
        0.5 * ray_color(&reflect_ray, world, depth - 1)
    } else {
        let t = 0.5 * (ray.direction.unify().y + 1.);
        (1. - t) * vec::Vec3::new(1., 1., 1.) + t * vec::Vec3::new(0.5, 0.7, 1.)
    }
}

fn main() {
    let mut buffer: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::new(WIDTH, HEIGHT);

    let s1 = sphere::Sphere::new(vec::Vec3::new(0., 0., -1.), 0.5);
    let s2 = sphere::Sphere::new(vec::Vec3::new(0., -100.5, -1.), 100.);

    let mut world = hit::HittableList::new();
    world.insert(&s1);
    world.insert(&s2);

    let camera = camera::Camera::new();

    buffer.enumerate_rows_mut().for_each(|(y, row)| {
        if y % 5 == 0 {
            println!("line: {}", y);
        }
        row.for_each(|(x, y, pixel)| {
            let rwidth = 1. / WIDTH as f32;
            let rheight = 1. / HEIGHT as f32;
            let color = (0..SAMPLE).fold(vec::Vec3::new(0., 0., 0.), |color, _| {
                let (u, v) = {
                    let mut rng = RNG.lock().unwrap();
                    (
                        x as f32 * rwidth + rng.gen_range(-rwidth, rwidth),
                        y as f32 * rheight + rng.gen_range(-rheight, rheight),
                    )
                };
                let ray = camera.get_ray(u, v);
                color + ray_color(&ray, &world, DEPTH)
            }) / SAMPLE as f32;
            *pixel = image::Rgb(color.pixel());
        });
    });

    buffer.save("emm.bmp").unwrap();
}
