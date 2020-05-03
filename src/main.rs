use image;

mod hittable;
mod ray;
mod vec;

use hittable::Hittable;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn ray_color(ray: ray::Ray) -> vec::Vec3 {
    let center = vec::Vec3::new(0., 0., -1.);
    let sphere = hittable::Sphere::new(center, 0.5);
    if let Some(hit_record) = sphere.hit(&ray, 0.1, 999.) {
        0.5 * (hit_record.normal + 1.)
    } else {
        let direction = ray.direction.unify();
        let t = 0.5 * (direction.y + 1.);
        (1. - t) * vec::Vec3::new(1., 1., 1.) + t * vec::Vec3::new(0.5, 0.7, 1.)
    }
}

fn main() {
    let mut buffer: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::new(WIDTH, HEIGHT);

    let lower_left_corner = vec::Vec3::new(-2., -1., -1.);
    let horizontal = vec::Vec3::new(4., 0., 0.);
    let vertical = vec::Vec3::new(0., 2., 0.);
    let origin = vec::Vec3::new(0., 0., 0.);

    buffer.enumerate_rows_mut().for_each(|(y, row)| {
        if y % 50 == 0 {
            println!("line: {}", y);
        }
        row.for_each(|(x, y, pixel)| {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let ray = ray::Ray::new(origin, direction);
            let color = ray_color(ray);
            *pixel = image::Rgb(color.pixel());
        });
    });

    buffer.save("emm.bmp").unwrap();
}
