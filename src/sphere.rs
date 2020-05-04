use crate::hit;
use crate::ray;
use crate::vec;

pub struct Sphere {
    pub center: vec::Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: vec::Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    /// Convenient function for generating hit::HitRecord for a sphere when we already know the t of hitting
    pub fn hit_record(&self, ray: &ray::Ray, t: f32) -> hit::HitRecord {
        let position = ray.at(t);
        let out_normal = position - self.center;
        let (front_face, normal) = if ray.direction.dot(out_normal) < 0. {
            (true, out_normal)
        } else {
            (false, -out_normal)
        };
        hit::HitRecord {
            position,
            normal,
            t,
            front_face,
        }
    }
}

impl hit::Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32) -> Option<hit::HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta >= 0. {
            // the smaller is better
            [(-half_b - delta.sqrt()) / a, (-half_b + delta.sqrt()) / a]
                .iter()
                .find(|&&t| t < tmax && t > tmin)
                .map(|&t| self.hit_record(ray, t))
        } else {
            None
        }
    }
}