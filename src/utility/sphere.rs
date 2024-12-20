use std::mem::discriminant;

use crate::utility::hittable::{HitRecord, Hittable};
use crate::utility::ray::Ray;
use crate::utility::vec3::{self,Point3, Vec3};


pub struct Sphere {
    center: Point3,
    radius: f64,
}


impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        return true;
    }

}

