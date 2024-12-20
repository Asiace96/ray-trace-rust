mod utility;
use utility::colors::Color;
use utility::colors;
use utility::vec3::{Point3, Vec3};
use utility::vec3;
use utility::ray::Ray;

use std::fs::File;
use std::io::Write;

fn create_file(path: &str) -> File {
    let fs = File::create(path)
        .expect("image.ppm should be included in this project");
    return fs;
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc  = center - r.origin();
    //let a = vec3::dot(r.direction(), r.direction());
    let a = r.direction().length_squared();
    //let b = -2.0 * vec3::dot(r.direction(), oc);
    let h = vec3::dot(r.direction(), oc);
    //let c = vec3::dot(oc,oc) - radius*radius;
    let c = oc.length_squared() - radius*radius;
    //let discriminant = b*b -4.0*a*c;
    let discriminant = h*h - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}


fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);
    if t > 0.0 {
        let n = vec3::unit_vector(r.at(t) - Vec3::new(0.0,0.0,-1.0));
        return 0.5*Color::new(n.x+1.0, n.y+1.0, n.z+1.0);
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
            let pixel_color = ray_color(r);
            colors::write_color(&mut output, pixel_color);
        }
    }
    println!("Done!");
    Ok(())
}

