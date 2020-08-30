use crate::hit;
use crate::rng::{random_in_hemisphere, random_in_unit_sphere, random_unit};
use crate::vec;

/// We assume normal is always at the same side with the in_direction. Normal and
/// in_direction are not unified.
fn reflect(in_direction: vec::Vec3, normal: vec::Vec3) -> vec::Vec3 {
    let normal = normal.unify();
    in_direction - 2. * in_direction.dot(normal) * normal
}

/// We assume normal is always at the same side with the in_direction. Normal and
/// in_direction are not unified.
fn refract(coefficient: f32, in_direction: vec::Vec3, normal: vec::Vec3) -> vec::Vec3 {
    let (r, n) = (in_direction.unify(), normal.unify());
    let cos_theta = -r.dot(n);
    if coefficient > 1. && (1. - cos_theta * cos_theta) * coefficient * coefficient > 1. {
        // All reflect
        reflect(in_direction, normal)
    } else {
        // refract
        // Christophe Schlick's hack
        let r0 = (1. - coefficient) / (1. + coefficient);
        let r0 = r0 * r0;
        let reflect_probability = r0 + (1. - r0) * (1. - cos_theta).powi(5);
        if random_unit() < reflect_probability {
            reflect(in_direction, normal)
        } else {
            let r_perpendicular = coefficient * (r + cos_theta * n);
            let r_parallel = -(1. - r_perpendicular.length_squared()).sqrt() * n;
            r_parallel + r_perpendicular
        }
    }
}

pub trait Material {
    /// Given ray_in direction and normal return attenuation and scattered
    /// direction. When reflected direction is illegal(e.g. For in-transparent
    /// material, reflected dot normal is positive), return None.
    fn scatter(
        &self,
        in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)>;
}

pub struct Metal {
    albedo: vec::Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(fuzz: f32, albedo: vec::Vec3) -> Self {
        if fuzz < 0. && fuzz >= 1. {
            panic!("invalid fuzz: not in [0, 1)")
        }
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        let reflected =
            reflect(in_direction, hit_record.normal) + self.fuzz * random_in_unit_sphere(); // fuzz a little bit
        Some((self.albedo, reflected))
    }
}

pub struct Lambertian {
    albedo: vec::Vec3,
}

impl Lambertian {
    pub fn new(albedo: vec::Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        Some((self.albedo, random_in_hemisphere(hit_record.normal)))
    }
}

pub struct Glass {
    eta: f32,
}

impl Glass {
    pub fn new(eta: f32) -> Self {
        Self { eta }
    }
}

impl Material for Glass {
    fn scatter(
        &self,
        in_direction: vec::Vec3,
        hit_record: &hit::HitRecord,
    ) -> Option<(vec::Vec3, vec::Vec3)> {
        // We assume two glass bell never overlaps, so always 1 / eta and eta /
        // 1 when in and out.
        let coefficient = if hit_record.front_face {
            // in
            1. / self.eta
        } else {
            // out
            self.eta
        };
        let refracted = refract(coefficient, in_direction, hit_record.normal);
        Some((vec::Vec3::new(1., 1., 1.), refracted))
    }
}

impl<T> From<T> for Box<dyn Material>
where
    T: 'static + Material,
{
    fn from(material: T) -> Self {
        Box::new(material)
    }
}
