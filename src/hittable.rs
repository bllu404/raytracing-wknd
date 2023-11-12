use std::ops::RangeInclusive;

use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    p: Point3,
    pub normal: Vec3, 
    t: f64,
    front_face: bool,
}

impl HitRecord {
    // Sets the hit record normal vector.
    // NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(ray.direction, *outward_normal) < 0.0;
        self.normal = if self.front_face {
             *outward_normal
        } else { 
            -*outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord { p: Point3::default(), normal: Vec3::default(), t: 0.0, front_face: false }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3, 
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere{
        Sphere{center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>, record: &mut HitRecord) -> bool {
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

        if !ray_t.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(&root) {
                return false;
            };
        }

        record.t = root;
        record.p = ray.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        true
    }
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {

    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>, record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = HitRecord::default();
        let mut hit_anything = false;
        let mut smallest_range_so_far = ray_t;

        for boxed_hittable in self {
            if (*boxed_hittable).hit(ray, smallest_range_so_far.clone(), &mut temp_record) {
                hit_anything = true;
                smallest_range_so_far = *smallest_range_so_far.start()..=temp_record.t;
                *record = temp_record;
            }
        }

        hit_anything
    }
}