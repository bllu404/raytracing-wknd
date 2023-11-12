use crate::utils::clamp;
use crate::vec::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn get_color(self, samples_per_pixel: i32) -> String {
        let r = self.x / samples_per_pixel as f64; 
        let g = self.y / samples_per_pixel as f64; 
        let b = self.z / samples_per_pixel as f64; 

        let intensity = 0.0..=0.999;

        format!(
            "{} {} {}\n", 
            (256.0 * clamp(&intensity, r)) as i32, 
            (256.0 * clamp(&intensity, g)) as i32, 
            (256.0 * clamp(&intensity, b)) as i32, 
        )
    }
}