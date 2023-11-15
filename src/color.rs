use image::Rgb;

use crate::utils::{clamp, gamma_to_linear, linear_to_gamma};
use crate::vec::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn get_rgb(self, samples_per_pixel: i32) -> Rgb<u8> {
        let r = linear_to_gamma(self.x / samples_per_pixel as f64);
        let g = linear_to_gamma(self.y / samples_per_pixel as f64);
        let b = linear_to_gamma(self.z / samples_per_pixel as f64);

        let intensity = 0.0..=0.999;

        Rgb([
            (256.0 * clamp(&intensity, r)) as u8,
            (256.0 * clamp(&intensity, g)) as u8,
            (256.0 * clamp(&intensity, b)) as u8,
        ])
    }

    pub fn get_color(r: f32, g: f32, b: f32) -> Color {
        Color::new(
            gamma_to_linear(r as f64 / 256.0),
            gamma_to_linear(g as f64 / 256.0),
            gamma_to_linear(b as f64 / 256.0),
        )
    }
}
