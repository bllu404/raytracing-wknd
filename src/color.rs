use crate::utils::{clamp, linear_to_gamma};
use crate::vec::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn get_color(self, samples_per_pixel: i32) -> String {
        let r = linear_to_gamma(self.x / samples_per_pixel as f64);
        let g = linear_to_gamma(self.y / samples_per_pixel as f64);
        let b = linear_to_gamma(self.z / samples_per_pixel as f64);

        let intensity = 0.0..=0.999;

        format!(
            "{} {} {}\n",
            (256.0 * clamp(&intensity, r)) as i32,
            (256.0 * clamp(&intensity, g)) as i32,
            (256.0 * clamp(&intensity, b)) as i32,
        )
    }
}
