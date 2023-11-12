use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

struct HitRecord {
    p: Point3,
    normal: Vec3, 
    t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, rayt_min: f64, rayt_max: f64, record: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Point3, 
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, rayt_min: f64, rayt_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center; // center of sphere to origin of vector
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(ray.direction, oc);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        // If the discriminant is greater than zero, then the ray intersects the sphere
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if !(rayt_min..=rayt_max).contains(root) {
            root = (-half_b + sqrtd) / a;
            if !(rayt_min..=rayt_max).contains(root) {
                return false;
            };
        }

        record.t = root;
        record.p = ray.at(record.t);
        record.normal = (record.p - self.center) / self.radius;

        true
    }
}