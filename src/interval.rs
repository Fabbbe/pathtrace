use std::default::Default;

use crate::utils::INFINITY;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

const EMPTY: Interval = Interval {min: INFINITY, max: -INFINITY};
const UNIVERSE: Interval = Interval {min: -INFINITY, max: INFINITY};

impl Default for Interval {
    // The default interval is empty
    fn default() -> Self {
        EMPTY
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval {min, max}
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }
}

