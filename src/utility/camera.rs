use crate::utility::ray::Ray;
use crate::utility::vec3::{self,Vec3, Point3};
use crate::utility::colors::{self,Color};
use crate::utility::interval::Interval;
use crate::utility::hittable::Hittable;
use crate::utility::common;
use std::fs::File;

use rayon::prelude::*;


#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height 
    pub image_width: i32, // Rendered image width in pixel count
    pub image_height: i32, // Rendred image height 
    pub samples_per_pixel: i32, // Count of random samples for each pixel
    pub max_depth: i32, // Maximum number of ray bounces into scene 
    pub vfov: f64, // Vertical viewing angle (field of view)
    pub look_from: Point3, // Point camera is looking from
    pub look_at: Point3, // Point camera is looking at 
    pub vup: Vec3, // Camera relative "up" direction
    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64, // Distance from camera lookrom point to plane of perfect focus
        pixel_sample_scale: f64, // Color scale factor for a sum of pixel sample 
        center: Point3, // Camera center 
        pixel00_loc: Point3, // Location of pixel 0, 0
        pixel_delta_u: Vec3, // Offset to pixel to the right 
        pixel_delta_v: Vec3, // Offset to pixel below 
        u: Vec3, v: Vec3, w: Vec3, // Camera frame basis vectors
        defocus_disk_u: Vec3, // defocus disk horizontal radius
        defocus_disk_v: Vec3, // defocus disk vertical radius
    
}


impl Camera {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn render(&mut self, world: &dyn Hittable, output: &mut File) {

        //Render
        for j in 0..self.image_height {
            println!("Scanlines remaining: {}", self.image_height - j);
            let pixel_colors: Vec<_> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0,0.0,0.0);
                    for _ in 0..self.samples_per_pixel {
                        let r = self.get_ray(i,j);
                        pixel_color += Self::ray_color(&r, self.max_depth, world);
                    }
                    pixel_color
                })
            .collect();
            for pixel_color in pixel_colors {
                colors::write_color(output, self.pixel_sample_scale * pixel_color);
            }
        }
        println!("Done!");
    }

    pub fn initialize(&mut self) {
        // Image
        self.image_height = (self.image_width as f64/ self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};

        // Render
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        // Camera
        self.center = self.look_from;
        let theta = common::degrees_to_radians(self.vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * self.aspect_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        self.w = vec3::unit_vector(self.look_from - self.look_at);
        self.u = vec3::unit_vector(vec3::cross(self.vup, self.w));
        self.v = vec3::cross(self.w, self.u);
         
        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = 
            self.center - (self.focus_dist * self.w) - viewport_u/2.0 -viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5*(self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * f64::tan(common::degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and direction at radnomly sampled 
        // point around the pixel location i,j

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
                         + ((i as f64 + offset.x) * self.pixel_delta_u)
                         + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the +-0.5 unit square
        return Vec3::new(common::random_double() - 0.5, common::random_double() - 0.5, 0.0);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Return a random point in the camera defocus disk 
        let p = vec3::random_in_unit_disk();
        return self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v); 
    }

    fn ray_color(r: &Ray, depth: i32,  world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth < 0 {
            return Color::from_float(0.0);
        }

        if let Some(hit_rec) = world.hit(r, Interval::new(0.001, common::INFINITY)) {
           if let Some(scatter_rec) = hit_rec.mat.scatter(r, &hit_rec) {
               return scatter_rec.attenuation * Self::ray_color(&scatter_rec.scattered, depth-1, world);
            }
            return Color::new(0.0,0.0,0.0);
        }

        let unit_direction: Vec3 = vec3::unit_vector(r.direction());
        let a = 0.5*(unit_direction.y + 1.0);
        return (1.0 - a) * Color::new(1.0,1.0,1.0) + a*Color::new(0.5,0.7,1.0);
    }
}



