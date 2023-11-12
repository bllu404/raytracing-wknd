mod color;
mod hittable;
mod ray;
mod vec;

use std::cmp::max;
use std::fs::File;
use std::io::{self, Write};

use crate::color::Color;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

fn get_ray_color(ray: &Ray) -> Color {
    let t = ray.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }

    let a = 0.5*(ray.direction.unit_vector().y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0; 
    let image_width: i32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height: i32 = max(1, (image_width as f64 / aspect_ratio) as i32);

    // Camera
    let focal_length: f64 = 1.0; 
    let viewport_height: f64 = 2.0; 
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // This positions the viewport such that a vector can be constructed that passes through 
    // `camera_center`, is orthogonal to the viewport, and passes through the center of the viewport
    let viewport_upper_left = camera_center - Point3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;

    // This is the top left pixel, each subsequent pixel will be calculated by adding a linear combination of 
    // `pixel_delta_u` and `pixel_delta_v` to this point
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut ppm_str = String::new();

    ppm_str.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    // Render

    for j in 0..image_height {
        
        if j != 0 {
            print!("\x1B[1A\x1B[K");
        }
        println!("Scanlines remaining: {}", image_height - j);
        io::stdout().flush().unwrap();

        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let color = get_ray_color(&r);

            ppm_str.push_str(&color.get_color());
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(ppm_str.as_bytes()).unwrap();

    print!("\x1B[1A\x1B[K");
    println!("All done!");
}
