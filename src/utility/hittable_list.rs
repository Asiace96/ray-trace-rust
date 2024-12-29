use crate::utility::hittable::{Hittable, HitRecord};
use crate::utility::ray::Ray;
use crate::utility::interval::Interval;



#[derive(Default)]
pub struct HittableList {
   objects: Vec<Box<dyn Hittable>>, 
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}


impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        return temp_rec;
    }
}
