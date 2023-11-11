mod color;
mod ray;
mod vec;

use std::fs::File;
use std::io::{self, Write};

use crate::color::Color;
use crate::ray::Ray;

fn main() {
    let image_w = 256; 
    let image_h = 256;

    let mut ppm_str = String::new();

    ppm_str.push_str(&format!("P3\n{} {}\n255\n", image_w, image_h));

    // Render

    for j in 0..image_h {
        
        if j != 0 {
            print!("\x1B[1A\x1B[K");
        }

        println!("Scanlines remaining: {}", image_h - j);
        io::stdout().flush().unwrap();

        for i in 0..image_w {
            let color = Color::new(
                i as f64 / (image_w - 1) as f64,
                j as f64 / (image_h - 1) as f64,
                0.25
            );

            ppm_str.push_str(&color.get_color());
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(ppm_str.as_bytes()).unwrap();

    print!("\x1B[1A\x1B[K");
    println!("All done!");
}
