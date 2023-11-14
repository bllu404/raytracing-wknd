mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod utils;
mod vec;

use std::sync::Arc;
use std::time::Instant;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HittableList, Sphere};
use crate::material::{Lambertian, Material, Metal};
use crate::vec::Point3;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground: Arc<dyn Material> = Arc::new(Lambertian{albedo: Color::new(0.8, 0.8, 0.0)});
    let material_center: Arc<dyn Material> = Arc::new(Lambertian{albedo: Color::new(0.7, 0.3, 0.3)});
    let material_top: Arc<dyn Material> = Arc::new(Metal{albedo: Color::new(0.7, 0.3, 0.3), f: 0.0});
    let material_left: Arc<dyn Material> = Arc::new(Metal{albedo: Color::new(0.5, 0.5, 0.5), f: 0.3});
    let material_right: Arc<dyn Material> = Arc::new(Metal{albedo: Color::new(0.8, 0.6, 0.2), f: 1.0});

    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -2.0), 100.0, material_ground)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -2.0), 0.5, material_center)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, -2.0), 0.5, material_top)));
    world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -2.0), 0.5, material_left)));
    world.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -2.0), 0.5, material_right)));

    let cam = Camera::new(16.0 / 9.0, 400);

    let start = Instant::now();
    cam.render(&world);
    println!("Render time: {:?}", start.elapsed());
}
