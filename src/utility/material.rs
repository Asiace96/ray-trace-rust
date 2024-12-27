use crate::utility::hittable::HitRecord;
use crate::utility::colors::Color;
use crate::utility::ray::Ray;
use crate::utility::vec3;



pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray
        ) -> bool;
}


pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(c: Color) -> Self {
        Lambertian { albedo: c }
    }
}


impl Material for Lambertian {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray
            ) -> bool {

            let mut scatter_direction = rec.normal + vec3::random_unit_vector();

            // Catch degenerate scatter direction 
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }

            *scattered = Ray::new(rec.p, scatter_direction);
            *attenuation = self.albedo;
            return true;
    }
}


pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(c: Color) -> Self {
        Metal { albedo: c }
    }
}


impl Material for Metal {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray
            ) -> bool {

            let reflected = vec3::reflect(r_in.direction(), rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            *attenuation = self.albedo;
            return true;
    }
}
