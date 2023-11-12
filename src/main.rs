mod camera;
mod color;
mod hittable;
mod ray;
mod utils;
mod vec;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable, HittableList, Sphere};
use crate::ray::Ray;
use crate::vec::{Point3};

fn get_ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut record = HitRecord:: default();

    if world.hit(ray, 0.0..=f64::INFINITY, &mut record) {
        return (record.normal + Point3::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let a = 0.5*(ray.direction.unit_vector().y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    
    // World
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(16.0/9.0, 400);
    cam.render(&world);
}
