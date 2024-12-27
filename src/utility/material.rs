use crate::utility::hittable::HitRecord;
use crate::utility::colors::Color;
use crate::utility::ray::Ray;


pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray
        ) -> bool;
}

