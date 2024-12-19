use std::io::Write;
use std::fs::File;
use crate::utility::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(output: &mut File, pixel_color: Color) {
    let r = (255.999 * pixel_color.x) as i32;
    let g = (255.999 * pixel_color.y) as i32;
    let b = (255.999 * pixel_color.z) as i32;
    writeln!(output, "{} {} {}", r, g, b).expect("writing color");
}
