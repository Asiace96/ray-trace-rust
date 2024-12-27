use std::io::Write;
use std::fs::File;
use crate::utility::vec3::Vec3;
use crate::utility::interval::Interval;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return f64::sqrt(linear_component);
    }
    return 0.0;
}

pub fn write_color(output: &mut File, pixel_color: Color) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    //Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255]
    let intensity = Interval::new(0.000, 0.999);
    let r = (256.0 * intensity.clamp(r)) as i32;
    let g = (256.0 * intensity.clamp(g)) as i32;
    let b = (256.0 * intensity.clamp(b)) as i32;
    writeln!(output, "{} {} {}", r, g, b).expect("writing color");
}
