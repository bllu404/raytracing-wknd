mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod utils;
mod vec;

use std::rc::Rc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HittableList, Sphere};
use crate::material::{Lambertian, Material};
use crate::vec::Point3;

fn main() {
    // World
    let mut world = HittableList::new();

    let mat: Rc<dyn Material> = Rc::new(Lambertian{albedo: Color::new(0.3, 1.0, 0.3)});

    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Rc::clone(&mat))));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::clone(&mat))));

    let cam = Camera::new(16.0 / 9.0, 400);

    cam.render(&world);
}
