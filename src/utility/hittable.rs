use std::sync::Arc;

use crate::utility::material::Material;
use crate::utility::ray::Ray;
use crate::utility::vec3::{self,Vec3, Point3};
use crate::utility::interval::Interval;


pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        //Sets the hit record normal vector
        //NOTE: the parameter 'outward_normal' is assumed to have unit length

        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else {-outward_normal};
    }
}


pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
