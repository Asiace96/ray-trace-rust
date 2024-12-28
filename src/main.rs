mod utility;
use utility::colors::{self,Color};
use utility::hittable::{HitRecord,Hittable};
use utility::hittable_list::HittableList;
use utility::vec3::{self,Point3,Vec3};
use utility::ray::Ray;
use utility::interval::Interval;
use utility::sphere::Sphere;
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
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    const VERTICAL_FIELD_OF_VIEW: f64 = 90.0;
    let LOOK_FROM = Point3::new(0.0,0.0,0.0);
    let LOOK_AT = Point3::new(0.0,0.0,-1.0);
    let VUP = Vec3::new(0.0,1.0,0.0);


    //World
    let mut world = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8,0.8,0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1,0.2,0.5)));
    let material_left   = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right  = Rc::new(Metal::new(Color::new(0.8,0.6,0.2), 0.0));


    world.add(Box::new(Sphere::new(
                       Point3::new(0.0, -100.5, -1.0),
                       100.0,
                       material_ground,
    )));

    world.add(Box::new(Sphere::new(
                       Point3::new(0.0, 0.0, -1.2),
                       0.5,
                       material_center,
    )));
    
    world.add(Box::new(Sphere::new(
                       Point3::new(-1.0, 0.0, -1.0),
                       0.5,
                       material_left,
    )));

    world.add(Box::new(Sphere::new(
                       Point3::new(-1.0, 0.0, -1.0),
                       0.4,
                       material_bubble,
    )));

    world.add(Box::new(Sphere::new(
                       Point3::new(1.0, 0.0, -1.0),
                       0.5,
                       material_right,
    )));

    //Camera
    let mut camera = Camera::new();
    camera.image_width = IMAGE_WIDTH;
    camera.aspect_ratio = ASPECT_RATIO;
    camera.samples_per_pixel = SAMPLES_PER_PIXEL;
    camera.max_depth = MAX_DEPTH;

    camera.vfov = VERTICAL_FIELD_OF_VIEW;
    camera.look_from = Point3::new(-2.0, 2.0, 1.0); // default - LOOK_FROM;
    camera.look_at = Point3::new(0.0, 0.0, -1.0); // default - LOOK_AT;
    camera.vup = Vec3::new(0.0, 1.0, 0.0); //default - VUP;

    camera.initialize();

    // Make file
    let mut output: File = create_file("image.ppm");
    writeln!(output, "P3")?;
    writeln!(output, "{} {}\n255", camera.image_width, camera.image_height)?;

    camera.render(&mut world, &mut output); 

    println!("Done!");
    Ok(())
}

