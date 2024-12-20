mod utility;
use utility::colors::{self,Color};
use utility::hittable::{HitRecord,Hittable};
use utility::hittable_list::HittableList;
use utility::vec3::{self,Point3,Vec3};
use utility::ray::Ray;
use utility::sphere::Sphere;
use utility::common;

use std::fs::File;
use std::io::Write;

fn create_file(path: &str) -> File {
    let fs = File::create(path)
        .expect("image.ppm should be included in this project");
    return fs;
}


fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, common::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
    }

    let unit_direction: Vec3 = vec3::unit_vector(r.direction());
    let a = 0.5*(unit_direction.y + 1.0);
    return (1.0 - a) * Color::new(1.0,1.0,1.0) + a*Color::new(0.5,0.7,1.0);
}

fn main() -> std::io::Result<()> {

    //Image

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    //Calcualte the image height and ensure that it's at least 1
    let mut image_height: i32 = (image_width as f64/ aspect_ratio) as i32;
    image_height = if image_height < 1 {1} else {image_height};

    //World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-30.5,1.0), 30.0)));

    //Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * aspect_ratio;
    let camera_center = Point3::new(0.0,0.0,0.0);
        
    //Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    //Calculate the horizontal and vertical delat vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    //Calculate the location of the upper left pixel
    let viewport_upper_left = 
        camera_center - Vec3::new(0.0,0.0,focal_length) - viewport_u/2.0 -viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5*(pixel_delta_u + pixel_delta_v);

    // Make file
    let mut output: File = create_file("image.ppm");
    writeln!(output, "P3")?;
    writeln!(output, "{} {}\n255", image_width, image_height)?;

    //Render
    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);
            colors::write_color(&mut output, pixel_color);
        }
    }
    println!("Done!");
    Ok(())
}

