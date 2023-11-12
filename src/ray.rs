use crate::vec::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3, 
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Ray {
        Ray{origin, direction}
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    pub fn hit_sphere(self, center: Point3, radius: f64) -> f64 {
        let oc = self.origin - center; // center of sphere to origin of vector
        let a = self.direction.length_squared();
        let half_b = Vec3::dot(self.direction, oc);
        let c = Vec3::dot(oc, oc) - radius * radius;
        // If the discriminant is greater than zero, then the ray intersects the sphere
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) /  a
        }
    }
}