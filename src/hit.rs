use crate::ray;
use crate::vec;

pub struct HitRecord {
    pub position: vec::Vec3,
    /// Attention: The normal not ensure to be unified, should be unified by
    /// the user.
    pub normal: vec::Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

pub struct HittableList<'a> {
    /// Currently we assume all object created before list is created and
    /// created in the main function.
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn insert(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold(
                (tmax, None),
                |(mut current_tmax, mut current_hit_record), &object| {
                    if let Some(hit_record) = object.hit(ray, tmin, tmax) {
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
