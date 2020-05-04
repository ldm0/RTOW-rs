use image;
use rand;

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec;

use rand::{Rng, SeedableRng};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;
const SAMPLE: u32 = 1000;
const DEPTH: u32 = 50;

fn ray_color(ray: &ray::Ray, world: &hit::HittableList) -> vec::Vec3 {
    if let Some(hit_record) = world.hit(&ray, 0.1, 999.) {
        0.5 * (hit_record.normal.unify() + 1.)
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

    let mut rng = rand::rngs::SmallRng::from_entropy();

    buffer.enumerate_rows_mut().for_each(|(y, row)| {
        if y % 50 == 0 {
            println!("line: {}", y);
        }
        row.for_each(|(x, y, pixel)| {
            let rwidth = 1. / WIDTH as f32;
            let rheight = 1. / HEIGHT as f32;
            let color = (0..SAMPLE).fold(vec::Vec3::new(0., 0., 0.), |color, _| {
                let u = x as f32 * rwidth + rng.gen_range(-rwidth, rwidth);
                let v = y as f32 * rheight + rng.gen_range(-rheight, rheight);
                let ray = camera.get_ray(u, v);
                color + ray_color(&ray, &world)
            }) / SAMPLE as f32;
            *pixel = image::Rgb(color.pixel());
        });
    });

    buffer.save("emm.bmp").unwrap();
}
