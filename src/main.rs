use image;

mod camera;
mod hit;
mod material;
mod ray;
mod rng;
mod sphere;
mod vec;

use rng::{random_color, random_unit};

const WIDTH: u32 = 160;
const HEIGHT: u32 = 90;
const SAMPLE: u32 = 300;
const DEPTH: u32 = 100;

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

    let ground_material = material::Lambertian::new(vec::Vec3::new(0.5, 0.5, 0.5));
    let ball0_material = material::Glass::new(1.5);
    let ball1_material = material::Lambertian::new(vec::Vec3::new(0.4, 0.2, 0.1));
    let ball2_material = material::Metal::new(0.0, vec::Vec3::new(0.7, 0.6, 0.5));

    let ground = sphere::Sphere::new(vec::Vec3::new(0., -1000., -1.), 1000., ground_material);
    let ball0 = sphere::Sphere::new(vec::Vec3::new(0., 1., 0.), 1.0, ball0_material);
    let ball1 = sphere::Sphere::new(vec::Vec3::new(-4., 1., 0.), 1.0, ball1_material);
    let ball2 = sphere::Sphere::new(vec::Vec3::new(4., 1., 0.), 1.0, ball2_material);

    let world = (-11..11).fold(
        hit::HittableList::new()
            .insert(ground)
            .insert(ball0)
            .insert(ball1)
            .insert(ball2),
        |world, a| {
            (-11..11).fold(world, |world, b| {
                let (a, b) = (a as f32, b as f32);
                let choose_mat = random_unit();
                let r1 = random_unit();
                let r2 = random_unit();
                let center = vec::Vec3::new(a + 0.9 * r1, 0.2, b + 0.9 * r2);
                if (center - vec::Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                    let material: Box<dyn material::Material> = if choose_mat < 0.8 {
                        // diffuse
                        let albedo = random_color();
                        Box::new(material::Lambertian::new(albedo))
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = 0.5 * random_color() + 0.5;
                        let fuzz = 0.5 * random_unit();
                        Box::new(material::Metal::new(fuzz, albedo))
                    } else {
                        // glass
                        Box::new(material::Glass::new(1.5))
                    };
                    world.insert(sphere::Sphere::new(center, 0.2, material))
                } else {
                    world
                }
            })
        },
    );

    let camera = camera::Camera::new(
        vec::Vec3::new(13., 2., 3.),
        vec::Vec3::new(0., 0., 0.),
        vec::Vec3::new(0., 1., 0.),
        20.,
        16. / 9.,
        0.1,
        10.,
    );

    buffer.enumerate_rows_mut().for_each(|(y, row)| {
        if y % 5 == 0 {
            println!("line: {}", y);
        }
        row.for_each(|(x, y, pixel)| {
            let (x, y) = (x as f32, y as f32);
            let color = (0..SAMPLE).fold(vec::Vec3::new(0., 0., 0.), |color, _| {
                let (u, v) = (
                    (x + random_unit()) / WIDTH as f32,
                    (y + random_unit()) / HEIGHT as f32,
                );
                let ray = camera.get_ray(u, v);
                color + ray_color(&ray, &world, DEPTH)
            }) / SAMPLE as f32;
            *pixel = image::Rgb(color.pixel());
        });
    });

    buffer.save("output.bmp").unwrap();
}
