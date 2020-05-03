use image;

mod ray;
mod vec;

const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn hit_sphere(center: vec::Vec3, radius: f32, ray: &ray::Ray) -> Option<f32> {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = ray.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let delta = half_b * half_b - a * c;
    if delta >= 0. {
        Some((-half_b - delta.sqrt()) / a)
    } else {
        None
    }
}

fn ray_color(ray: ray::Ray) -> vec::Vec3 {
    let center = vec::Vec3::new(0., 0., -1.);
    if let Some(t) = hit_sphere(center, 0.5, &ray) {
        let normal = (ray.at(t) - center).unify();
        0.5 * (normal + 1.)
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

    buffer
        .enumerate_rows_mut()
        .map(|(y, row)| {
            if y % 50 == 0 {
                println!("line: {}", y);
            }
            row.map(|(x, y, pixel)| {
                let u = x as f32 / WIDTH as f32;
                let v = y as f32 / HEIGHT as f32;
                let direction = lower_left_corner + u * horizontal + v * vertical;
                let ray = ray::Ray::new(origin, direction);
                let color = ray_color(ray);
                *pixel = image::Rgb(color.pixel());
            })
            .count();
        })
        .count();

    buffer.save("emm.bmp").unwrap();
}
