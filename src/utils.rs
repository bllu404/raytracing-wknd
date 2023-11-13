use std::ops::RangeInclusive;

use rand::distributions::{Distribution, Uniform};

pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn get_random_f64() -> f64 {
    // Range [0.0, 1.0)
    Uniform::new(0.0, 1.0).sample(&mut rand::thread_rng())
}

pub fn get_random_f64_custom(min: f64, max: f64) -> f64 {
    Uniform::new(min, max).sample(&mut rand::thread_rng())
}

pub fn clamp(range: &RangeInclusive<f64>, x: f64) -> f64 {
    x.clamp(*range.start(), *range.end())
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
