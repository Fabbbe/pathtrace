use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::utils;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    //p: [f64; 3],
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// + operator
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: v.x * self,
            y: v.y * self,
            z: v.z * self,
        }
    }
}

// Can only divide a vector by a float, not other way around
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, dividend: f64) -> Self {
        self*(1.0/dividend)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }

    pub fn length_squared(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x 
        + self.y * other.y
        + self.z * other.z
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    // Returns a vector where x, y and z are randomized from 0 to 1
    pub fn rand() -> Self {
        Vec3 {
            x: utils::rand_float(),
            y: utils::rand_float(),
            z: utils::rand_float(),
        }
    }

    pub fn rand_in(min: f64, max: f64) -> Self {
        Vec3 {
            x: utils::rand_float_in(min, max),
            y: utils::rand_float_in(min, max),
            z: utils::rand_float_in(min, max),
        }
    }

    pub fn rand_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::rand_in(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn rand_unit_vector() -> Self {
        Self::rand_in_unit_sphere().unit()
    }

    pub fn rand_on_hemisphere(normal: Vec3) -> Self {
        let p = Self::rand_unit_vector();
        if p.dot(normal) > 0.0 {
            return p
        }
        -p
    }
}

// TODO: write some tests 

