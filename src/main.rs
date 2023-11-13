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
use crate::material::{Lambertian, Material, Metal};
use crate::vec::Point3;

fn main() {
    // World
    let mut world = HittableList::new();

    let mat: Rc<dyn Material> = Rc::new(Lambertian{albedo: Color::new(0.3, 1.0, 0.3)});

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian{albedo: Color::new(0.8, 0.8, 0.0)});
    let material_center: Rc<dyn Material> = Rc::new(Lambertian{albedo: Color::new(0.7, 0.3, 0.3)});
    let material_left: Rc<dyn Material> = Rc::new(Metal{albedo: Color::new(0.8, 0.8, 0.8)});
    let material_right: Rc<dyn Material> = Rc::new(Metal{albedo: Color::new(0.8, 0.6, 0.2)});

    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let cam = Camera::new(16.0 / 9.0, 800);

    cam.render(&world);
}
