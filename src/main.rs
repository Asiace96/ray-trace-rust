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

use std::fs::File;
use std::io::Write;

fn create_file(path: &str) -> File {
    let fs = File::create(path)
        .expect("image.ppm should be included in this project");
    return fs;
}

fn main() -> std::io::Result<()> {
    //World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-30.5,1.0), 30.0)));
    
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

