use std::sync::Arc;

use crate::color::Color;
use crate::hittable::{Triangle, Translation};
use crate::material::Lambertian;
use crate::vec::{Point3, Vec3};

pub fn load_triangles(filename: &str) -> Vec<Box<Triangle>> {
    let scenes = easy_gltf::load(filename).expect("Failed to load glTF");

    let wizard = &scenes[0].models[0];

    let triangles = wizard.triangles().expect("Failed to get triangles");
    let material = wizard.material();

    let triangles: Vec<_> = wizard
        .triangles()
        .expect("Failed to get triangles")
        .iter()
        .map(|triangle| {
            let p1 = Point3::new(
                triangle[0].position[0] as f64,
                triangle[0].position[1] as f64,
                triangle[0].position[2] as f64,
            );
            let p2 = Point3::new(
                triangle[1].position[0] as f64,
                triangle[1].position[1] as f64,
                triangle[1].position[2] as f64,
            );
            let p3 = Point3::new(
                triangle[2].position[0] as f64,
                triangle[2].position[1] as f64,
                triangle[2].position[2] as f64,
            );
            let normal = Vec3::new(
                triangle[0].normal.x as f64,
                triangle[0].normal.y as f64,
                triangle[0].normal.z as f64,
            );

            let color = material.get_base_color(triangle[0].tex_coords);
            let albedo = Color::get_color(color.x, color.y, color.z);

            let mat = Lambertian { albedo };
            let triangle = Triangle::new_with_normal(p1, p2, p3, normal, Arc::new(mat))
                .scale(4.0)
                .translate(Translation::Down(7.0))
                .translate(Translation::Backward(3.0));
            Box::new(triangle)
        })
        .collect();
    
    triangles
}
