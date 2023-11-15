use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::get_random_f64;
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
        if scattered.direction.dot(&record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refractive_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = unit_direction.dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let new_ray_direction = if refraction_ratio * sin_theta > 1.0
        //|| reflectance(cos_theta, refraction_ratio) > get_random_f64()
        {
            unit_direction.reflect(&record.normal)
        } else {
            unit_direction.refract(&record.normal, refraction_ratio)
        };

        Some((attenuation, Ray::new(record.p, new_ray_direction)))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    let pow1 = 1.0 - cosine;
    let pow2 = pow1 * pow1;
    r0 + (1.0 - r0) * pow2 * pow2 * pow1
}
