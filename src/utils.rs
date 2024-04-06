
use std::f64::consts::PI;
pub use std::f64::INFINITY;

#[inline]
fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
