use std::f64::consts::PI;
pub use std::f64::INFINITY;

use rand::prelude::*;

#[inline]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn rand_float() -> f64 {
    random::<f64>()
}

#[inline]
pub fn rand_float_in(min: f64, max: f64) -> f64 {
    random::<f64>()*(max-min) + min
}
