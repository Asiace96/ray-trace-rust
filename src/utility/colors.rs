use std::io::Write;
use std::fs::File;
use crate::utility::vec3::Vec3;
use crate::utility::interval::Interval;

pub type Color = Vec3;

pub fn write_color(output: &mut File, pixel_color: Color) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Translate the [0,1] component values to the byte range [0,255]
    let intensity = Interval::new(0.000, 0.999);
    let r = (256.0 * intensity.clamp(r)) as i32;
    let g = (256.0 * intensity.clamp(g)) as i32;
    let b = (256.0 * intensity.clamp(b)) as i32;
    writeln!(output, "{} {} {}", r, g, b).expect("writing color");
}
