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
use utility::material::{Metal, Lambertian};

use std::fs::File;
use std::io::Write;
use std::rc::Rc;

fn create_file(path: &str) -> File {
    let fs = File::create(path)
        .expect("image.ppm should be included in this project");
    return fs;
}

fn main() -> std::io::Result<()> {
    //World
    let mut world = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8,0.8,0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1,0.2,0.5)));
    let material_left   = Rc::new(Metal::new(Color::new(0.8,0.8,0.8)));
    let material_right  = Rc::new(Metal::new(Color::new(0.8,0.6,0.2)));


    world.add(Box::new(Sphere::new(
                       Point3::new(0.0, -100.5, -1.0),
                       100.0,
                       material_ground,
    )));

    world.add(Box::new(Sphere::new(
                       Point3::new(0.0, 0.0, -1.0),
                       0.5,
                       material_center,
    )));
    
    world.add(Box::new(Sphere::new(
                       Point3::new(-1.0, 0.0, -1.0),
                       0.5,
                       material_left,
    )));

    world.add(Box::new(Sphere::new(
                       Point3::new(1.0, 0.0, -1.0),
                       0.5,
                       material_right,
    )));

    //Camera
    let mut camera = Camera::new();
    camera.initialize();

    // Make file
    let mut output: File = create_file("image.ppm");
    writeln!(output, "P3")?;
    writeln!(output, "{} {}\n255", camera.image_width, camera.image_height)?;

    camera.render(&mut world, &mut output); 

    println!("Done!");
    Ok(())
}

