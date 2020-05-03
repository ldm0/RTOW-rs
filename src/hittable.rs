use crate::ray;
use crate::vec;

pub struct HitRecord {
    pub position: vec::Vec3,
    pub normal: vec::Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: vec::Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: vec::Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta >= 0. {
            Some((-half_b - delta.sqrt()) / a)
                .filter(|&t| t < tmax && t > tmin)
                .map(|t| {
                    let position = ray.at(t);
                    let normal = (position - self.center).unify();
                    HitRecord { position, normal }
                })
        } else {
            None
        }
    }
}
