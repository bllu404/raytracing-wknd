mod camera;
mod color;
mod hittable;
mod material;
mod models;
mod ray;
mod utils;
mod vec;

use std::sync::Arc;
use std::time::Instant;

use camera::Camera;
use color::Color;
use hittable::{HittableList, Sphere, Translation, Triangle};
use material::{Dielectric, Lambertian, Material, Metal};
use models::load_triangles;
use vec::Point3;

fn main() {
    // World
    let mut world = HittableList::new();

    let matte_grey: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Color::new(0.3, 0.3, 0.35),
    });
    let matte_pink: Arc<dyn Material> = Arc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let metallic_pink: Arc<dyn Material> = Arc::new(Metal {
        albedo: Color::new(0.7, 0.3, 0.3),
        f: 0.0,
    });

    let metallic_green: Arc<dyn Material> = Arc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.0),
        f: 0.0,
    });

    let fuzzy_metallic_grey: Arc<dyn Material> = Arc::new(Metal {
        albedo: Color::new(0.5, 0.5, 0.5),
        f: 0.3,
    });
    let fuzzy_metallic_yellow: Arc<dyn Material> = Arc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        f: 1.0,
    });
    let glass: Arc<dyn Material> = Arc::new(Dielectric {
        refractive_index: 1.5,
    });

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -2.0),
        100.0,
        matte_grey,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -2.0),
        0.5,
        matte_pink.clone(),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, -2.0),
        0.5,
        metallic_pink.clone(),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -2.0),
        0.5,
        fuzzy_metallic_grey,
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -2.0),
        0.5,
        fuzzy_metallic_yellow,
    )));

    world.push(Box::new(Sphere::new(
        Point3::new(-0.5, -0.35, -1.0),
        0.15,
        glass,
    )));

    world.push(Box::new(
        Triangle::new(
            Point3::new(-1.5, 0.25, -1.5),
            Point3::new(-0.8, 0.25, -3.5),
            Point3::new(-1.15, 2.0, -2.5),
            metallic_green.clone(),
        )
        .translate(Translation::Left(0.75)),
    ));


    // 3D model rendering
    /* 
    let duck = load_triangles("duck.glb");
    println!("num triangles: {}", duck.len());
    for triangle in duck {
        world.push(triangle);
    }
    */
    let cam = Camera::new(16.0 / 9.0, 400);

    let start = Instant::now();
    cam.render(&world);
    println!("Time to render: {}", start.elapsed().as_secs());
}
