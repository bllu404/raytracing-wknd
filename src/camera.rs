use std::cmp::max;
use std::fs::File;
use std::io::{self, Write};

use crate::color::Color;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::utils::get_random_f64;
use crate::vec::{Point3, Vec3};

const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Camera {
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
        let viewport_upper_left = camera_center
            - Point3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        // This is the top left pixel, each subsequent pixel will be calculated by adding a linear combination of
        // `pixel_delta_u` and `pixel_delta_v` to this point
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            image_width,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        let mut ppm_str = String::new();

        ppm_str.push_str(&format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));

        // Render

        for j in 0..self.image_height {
            if j != 0 {
                print!("\x1B[1A\x1B[K");
            }
            println!("Scanlines remaining: {}", self.image_height - j);
            io::stdout().flush().unwrap();

            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, world, MAX_DEPTH)
                }

                ppm_str.push_str(&pixel_color.get_color(SAMPLES_PER_PIXEL));
            }
        }

        let mut file = File::create("image.ppm").unwrap();
        file.write_all(ppm_str.as_bytes()).unwrap();

        print!("\x1B[1A\x1B[K");
        println!("All done!");
    }

    fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = world.hit(ray, 0.001..=f64::INFINITY) {
            if let Some((attenuation, scattered)) = (*record.mat.clone().unwrap()).scatter(ray, record) {
                return attenuation * Self::ray_color(&scattered, world, depth - 1)
            }
            return Color::default();
        }

        let a = 0.5 * (ray.direction.unit_vector().y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center =
            self.pixel00_loc + self.pixel_delta_u * i as f64 + self.pixel_delta_v * j as f64;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + get_random_f64();
        let py = -0.5 + get_random_f64();
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }
}
