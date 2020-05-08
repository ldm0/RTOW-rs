use crate::hit;
use crate::material;
use crate::ray;
use crate::vec;

pub struct Sphere<'a> {
    pub center: vec::Vec3,
    pub radius: f32,
    pub material: &'a dyn material::Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: vec::Vec3, radius: f32, material: &'a dyn material::Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// Convenient function for generating hit::HitRecord for a sphere when we
    /// already know the t of hit.
    pub fn hit_record(&self, ray: &ray::Ray, t: f32) -> hit::HitRecord {
        let position = ray.at(t);
        // When radius is negative we get reversed normal, where we can do the hollow glass ball trick
        let out_normal = (position - self.center) / self.radius;
        // We need normal in the same side of in_ray to ensure Lambertian works correctly.
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
            material: self.material,
        }
    }
}

impl<'a> hit::Hittable for Sphere<'a> {
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
