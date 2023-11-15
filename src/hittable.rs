use std::ops::RangeInclusive;
use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Sets the hit record normal vector.
    // NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn get_face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        (front_face, normal)
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: None,
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord> {
        let oc = ray.origin - self.center; // center of sphere to origin of vector
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        // If the discriminant is greater than zero, then the ray intersects the sphere
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;

        if !ray_t.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            };
        }

        let p = ray.at(root);

        let outward_normal = (p - self.center) / self.radius;
        let (front_face, normal) = HitRecord::get_face_normal(ray, &outward_normal);

        Some(HitRecord {
            t: root,
            p,
            normal,
            mat: Some(Arc::clone(&self.mat)),
            front_face,
        })
    }
}

pub struct Triangle {
    p1: Point3,
    p2: Point3,
    p3: Point3,
    mat: Arc<dyn Material>,
    p1_p2: Vec3,
    p1_p3: Vec3,
    p2_p3: Vec3,
    normal: Vec3,
    d: f64,
}

pub enum Translation {
    Left(f64),
    Right(f64),
    Up(f64),
    Down(f64),
    Forward(f64),
    Backward(f64),
}

impl Triangle {
    pub fn new(p1: Point3, p2: Point3, p3: Point3, mat: Arc<dyn Material>) -> Triangle {
        let p1_p2 = p2 - p1;
        let p1_p3 = p3 - p1;
        let p2_p3 = p3 - p2;
        let normal = (p1_p3).cross(&p1_p2).unit_vector();

        // Finding the D such that the equation normal . point_on_plane = D is satisfied
        let d = normal.dot(&(p1));

        Triangle {
            p1,
            p2,
            p3,
            mat,
            p1_p2,
            p1_p3,
            p2_p3,
            normal,
            d,
        }
    }

    pub fn new_with_normal(
        p1: Point3,
        p2: Point3,
        p3: Point3,
        normal: Vec3,
        mat: Arc<dyn Material>,
    ) -> Triangle {
        let p1_p2 = p2 - p1;
        let p1_p3 = p3 - p1;
        let p2_p3 = p3 - p2;
        let normal = (p1_p3).cross(&p1_p2).unit_vector();

        // Finding the D such that the equation normal . point_on_plane = D is satisfied
        let d = normal.dot(&(p1));

        Triangle {
            p1,
            p2,
            p3,
            mat,
            p1_p2,
            p1_p3,
            p2_p3,
            normal,
            d,
        }
    }

    pub fn translate(self, translation: Translation) -> Self {
        let translation_vec = match translation {
            Translation::Left(amt) => Vec3::new(-amt, 0.0, 0.0),
            Translation::Right(amt) => Vec3::new(-amt, 0.0, 0.0),
            Translation::Up(amt) => Vec3::new(0.0, amt, 0.0),
            Translation::Down(amt) => Vec3::new(0.0, -amt, 0.0),
            Translation::Forward(amt) => Vec3::new(0.0, 0.0, amt),
            Translation::Backward(amt) => Vec3::new(0.0, 0.0, -amt),
        };
        Triangle::new(
            self.p1 + translation_vec,
            self.p2 + translation_vec,
            self.p3 + translation_vec,
            self.mat,
        )
    }

    pub fn scale(self, scalar: f64) -> Self {
        Triangle::new_with_normal(
            self.p1 * scalar,
            self.p2 * scalar,
            self.p3 * scalar,
            self.normal,
            self.mat,
        )
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord> {
        let normal = if self.normal.dot(&ray.direction) > 0.0 {
            -self.normal
        } else {
            self.normal
        };

        let discriminant = normal.dot(&ray.direction);

        // If the ray is parallel to the plane containing the triangle,
        // then it does not intersect the triangle
        if discriminant >= 0.0 {
            return None;
        }

        let t = (self.d - normal.dot(&ray.origin)) / discriminant;

        if !ray_t.contains(&t) {
            return None;
        }

        let point_of_intersection = ray.origin + ray.direction * t;

        let double_triangle_area = self.p1_p3.cross(&self.p1_p2).length();

        let p1_poi = point_of_intersection - self.p1;
        let alpha = self.p1_p3.cross(&p1_poi).length();
        let beta = self.p1_p2.cross(&p1_poi).length();
        let gamma = self
            .p2_p3
            .cross(&(point_of_intersection - self.p2))
            .length();

        if alpha + beta + gamma <= double_triangle_area {
            Some(HitRecord {
                p: point_of_intersection,
                normal,
                mat: Some(self.mat.clone()),
                t,
                front_face: true,
            })
        } else {
            None
        }
    }
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord> {
        let mut smallest_range_so_far = ray_t;

        let mut record = None;

        for hittable in self {
            if let Some(hittable_record) = (*hittable).hit(ray, smallest_range_so_far.clone()) {
                smallest_range_so_far = *smallest_range_so_far.start()..=hittable_record.t;
                record = Some(hittable_record);
            }
        }
        record
    }
}
