mod utility;
use utility::colors::{self,Color};
use utility::hittable::{HitRecord,Hittable};
use utility::hittable_list::HittableList;
use utility::vec3::{self,Point3,Vec3};
use utility::ray::Ray;
use utility::interval::Interval;
use utility::sphere::{self, Sphere};
use utility::common;
use utility::camera::Camera;
use utility::material::{Dielectric, Lambertian, Metal};

use std::fs::File;
use std::io::Write;
use std::rc::Rc;

fn create_file(path: &str) -> File {
    let fs = File::create(path)
        .expect("image.ppm should be included in this project");
    return fs;
}

fn main() -> std::io::Result<()> {
    // Constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1200;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;
    const VERTICAL_FIELD_OF_VIEW: f64 = 90.0;
    let LOOK_FROM = Point3::new(0.0,0.0,0.0);
    let LOOK_AT = Point3::new(0.0,0.0,-1.0);
    let VUP = Vec3::new(0.0,1.0,0.0);
    const DEFOCUS_ANGLE: f64 = 0.0;
    const FOCUS_DIST: f64 = 10.0;


    //World
    let mut world = HittableList::new();
    let groud_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
                       Point3::new(0.0, -1000.0, 0.0),
                       1000.0,
                       groud_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random_double();
            let center = Point3::new(a as f64 + 0.9*common::random_double(), 0.2, b as f64 + 0.9*common::random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse 
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
                       Point3::new(0.0, 1.0, 0.0),
                       1.0,
                       material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
                       Point3::new(-4.0, 1.0, 0.0),
                       1.0,
                       material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
                       Point3::new(4.0, 1.0, 0.0),
                       1.0,
                       material3,
    )));

    //Camera
    let mut camera = Camera::new();
    camera.image_width = IMAGE_WIDTH;
    camera.aspect_ratio = ASPECT_RATIO;
    camera.samples_per_pixel = SAMPLES_PER_PIXEL;
    camera.max_depth = MAX_DEPTH;

    camera.vfov = 20.0; // VERTICAL_FIELD_OF_VIEW
    camera.look_from = Point3::new(13.0, 2.0, 3.0); // LOOK_FROM;
    camera.look_at = Point3::new(0.0, 0.0, 0.0); // LOOK_AT;
    camera.vup = Vec3::new(0.0, 1.0, 0.0); // VUP;

    camera.defocus_angle = 0.6; // DEFOCUS_ANGLE
    camera.focus_dist = FOCUS_DIST;
    camera.initialize();

    // Make file
    let mut output: File = create_file("image.ppm");
    writeln!(output, "P3")?;
    writeln!(output, "{} {}\n255", camera.image_width, camera.image_height)?;

    camera.render(&mut world, &mut output); 

    println!("Done!");
    Ok(())
}

