mod camera;
mod color;
mod hittable;
mod ray;
mod utils;
mod vec;

use crate::camera::Camera;
use crate::hittable::{HittableList, Sphere};
use crate::vec::Point3;

fn main() {
    // World
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(16.0 / 9.0, 400);

    cam.render(&world);
}
