use std::sync::Arc;

use crate::utility::hittable::{HitRecord, Hittable};
use crate::utility::material::Material;
use crate::utility::ray::Ray;
use crate::utility::vec3::{self,Point3, Vec3};
use crate::utility::interval::Interval;


pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}


impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,

        }
    }
}


impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.material.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        return Some(rec);
    }

}

