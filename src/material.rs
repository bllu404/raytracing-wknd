use std::cmp::min;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;


pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, record: HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }

        Some((self.albedo, Ray::new(record.p, scatter_direction)))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub f: f64, // fuzz factor
}



impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction.reflect(&record.normal);

        let scattered = Ray::new(record.p, reflected + Vec3::random_unit_vector() * self.f);
        if Vec3::dot(&scattered.direction, &record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}