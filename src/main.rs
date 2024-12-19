mod utility;
use crate::utility::colors::Color;
use crate::utility::colors::write_color;

use std::fs::File;
use std::io::Write;

fn create_file(path: &str) -> File {
    let fs = File::create(path)
        .expect("image.ppm should be included in this project");
    return fs;
}

fn main() -> Result<(), std::io::Error> {

    let image_width: i32 = 256;
    let image_height: i32 = 256;
        
    let mut output: File = create_file("image.ppm");
    writeln!(output, "P3")?;
    writeln!(output, "{} {}\n255", image_width, image_height)?;

    for j in 0..256 {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..256 {
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0.0;
            let pixel_color = Color::new(r,g,b);
            write_color(&mut output, pixel_color);
        }
    }
    println!("Done!");
    Ok(())
}

