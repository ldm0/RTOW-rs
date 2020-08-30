use crate::material;
use crate::ray;
use crate::vec;

pub struct HitRecord<'a> {
    pub position: vec::Vec3,
    /// Attention: The normal not ensure to be unified, should be unified by
    /// the user.
    pub normal: vec::Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn material::Material,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    /// Currently we assume all object created before list is created and
    /// created in the main function.
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn insert<T: 'static + Hittable>(mut self, object: T) -> Self {
        self.objects.push(Box::new(object));
        self
    }

    pub fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold(
                (tmax, None),
                |(mut current_tmax, mut current_hit_record), object| {
                    if let Some(hit_record) = object.hit(ray, tmin, tmax) {
                        // find the smallest t
                        if hit_record.t < current_tmax {
                            current_tmax = hit_record.t;
                            current_hit_record = Some(hit_record);
                        }
                    }
                    (current_tmax, current_hit_record)
                },
            )
            .1
    }
}
