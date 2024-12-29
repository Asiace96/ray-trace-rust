use crate::utility::hittable::HitRecord;
use crate::utility::colors::Color;
use crate::utility::ray::Ray;
use crate::utility::vec3;
use crate::utility::common;


pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        ) -> Option<ScatterRecord>;
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
            _r_in: &Ray,
            rec: &HitRecord,
            ) -> Option<ScatterRecord> {

            let mut scatter_direction = rec.normal + vec3::random_unit_vector();

            // Catch degenerate scatter direction 
            if scatter_direction.near_zero() {
                scatter_direction = rec.normal;
            }

            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: Ray::new(rec.p, scatter_direction),
            })

    }
}


pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(c: Color, f: f64) -> Self {
        Metal { albedo: c, 
                fuzz: if f < 1.0 {f} else {1.0}, }
    }
}


impl Material for Metal {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            ) -> Option<ScatterRecord> {

            let reflected = vec3::reflect(r_in.direction(), rec.normal);
            let scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_unit_vector());
            if vec3::dot(scattered.direction(), rec.normal) > 0.0 {
                Some(ScatterRecord {
                    attenuation: self.albedo,
                    scattered: Ray::new(rec.p, reflected),
                })
            } else {
                None
            }
    }
}


pub struct Dielectric {
    refraction_index: f64, // Refractive index in vaccum or air , or the ratio
                           // of the material's refractive index over the refractive index 
                           // of the enclosing media
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric {refraction_index: ri }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0*r0;
        return r0 + (1.0 - r0)*f64::powf(1.0 - cosine, 5.0);
    }
}

impl Material for Dielectric {
    fn scatter(
            &self,
            r_in: &Ray,
            rec: &HitRecord,
            ) -> Option<ScatterRecord> {
           
            let refraction_ratio = if rec.front_face {
                1.0 / self.refraction_index
            } else {
                self.refraction_index
            };

            let unit_direction = vec3::unit_vector(r_in.direction());
            let cos_theta = f64::min(vec3::dot(-unit_direction, rec.normal),1.0);
            let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);
            let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
            let direction = if cannot_refract ||
                Self::reflectance(cos_theta, refraction_ratio) > common::random_double() 
            {
                vec3::reflect(unit_direction, rec.normal)
            } else {
                vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };

            Some(ScatterRecord {
                attenuation: Color::new(1.0,1.0,1.0),
                scattered: Ray::new(rec.p, direction),
            })
    }
}

