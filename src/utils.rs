use image::Rgb;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{Color, Interval};

pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn linear_to_gamma(linear_cmp: f64) -> f64 {
    linear_cmp.sqrt()
}

pub fn write_color(color: Color, samples_per_pixel: i64) -> Rgb<u8> {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    let scale = 1. / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0., 0.999);
    const RGB: f64 = 256.;
    let x = (RGB * intensity.clamp(r)) as i64;
    let y = (RGB * intensity.clamp(g)) as i64;
    let z = (RGB * intensity.clamp(b)) as i64;

    Rgb([x as u8, y as u8, z as u8])
}

pub fn progress_bar(width: i64, height: i64) -> ProgressBar {
    let pixels = (width * height) as u64;

    ProgressBar::new(pixels).with_style(
        ProgressStyle::with_template(
            "[{percent} %]  {wide_bar:.cyan/blue} {pos:>7}/{len:7} \n{msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    )
}
