use image;

mod camera;
mod hit;
mod material;
mod ray;
mod rng;
mod sphere;
mod vec;

use rand::Rng;
use rng::RNG;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const SAMPLE: u32 = 100;
const DEPTH: u32 = 50;

fn ray_color(ray: &ray::Ray, world: &hit::HittableList, depth: u32) -> vec::Vec3 {
    if depth == 0 {
        vec::Vec3::new(0., 0., 0.)
    } else if let Some(hit_record) = world.hit(&ray, 0.001, std::f32::MAX) {
        if let Some((attenuation, reflect)) =
            hit_record.material.scatter(ray.direction, &hit_record)
        {
            let ray = ray::Ray::new(hit_record.position, reflect);
            attenuation * ray_color(&ray, world, depth - 1)
        } else {
            // ERROR
            vec::Vec3::new(0., 0., 0.)
        }
    } else {
        let t = 0.5 * (ray.direction.unify().y + 1.);
        (1. - t) * vec::Vec3::new(1., 1., 1.) + t * vec::Vec3::new(0.5, 0.7, 1.)
    }
}

fn main() {
    let mut buffer: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::new(WIDTH, HEIGHT);

    let l1 = material::Lambertian::new(vec::Vec3::new(0.7, 0.3, 0.3));
    let l2 = material::Lambertian::new(vec::Vec3::new(0.8, 0.8, 0.0));
    let m1 = material::Metal::new(1.0, vec::Vec3::new(0.8, 0.6, 0.2));
    let m2 = material::Metal::new(0.3, vec::Vec3::new(0.8, 0.8, 0.8));
    let g1 = material::Glass::new(1.5);

    let s1 = sphere::Sphere::new(vec::Vec3::new(0., 0., -1.), 0.5, &l1);
    let s2 = sphere::Sphere::new(vec::Vec3::new(0., -100.5, -1.), 100., &l2);
    let s3 = sphere::Sphere::new(vec::Vec3::new(-1., 0., -1.), 0.5, &m2);
    let s4 = sphere::Sphere::new(vec::Vec3::new(1., 0., -1.), 0.5, &g1);
    let s5 = sphere::Sphere::new(vec::Vec3::new(1., 0., -1.), -0.45, &g1);

    let mut world = hit::HittableList::new();
    world.insert(&s1);
    world.insert(&s2);
    world.insert(&s3);
    world.insert(&s4);
    world.insert(&s5);

    let camera = camera::Camera::new(
        vec::Vec3::new(0., 0., 0.5),
        vec::Vec3::new(0., 0., -1.),
        vec::Vec3::new(0., 1., 0.),
        60.,
        60.,
    );

    buffer.enumerate_rows_mut().for_each(|(y, row)| {
        if y % 50 == 0 {
            println!("line: {}", y);
        }
        row.for_each(|(x, y, pixel)| {
            let (rwidth, rheight) = (1. / WIDTH as f32, 1. / HEIGHT as f32);
            let (x, y) = (x as f32, y as f32);
            let color = (0..SAMPLE).fold(vec::Vec3::new(0., 0., 0.), |color, _| {
                let (u, v) = {
                    let mut rng = RNG.lock().unwrap();
                    (
                        x * rwidth + rng.gen_range(-rwidth, rwidth),
                        y * rheight + rng.gen_range(-rheight, rheight),
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
