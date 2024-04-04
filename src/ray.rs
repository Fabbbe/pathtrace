use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

// This could be made to be generic float
impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {orig, dir}
    }

    // Gets the place at a specific distance
    pub fn at(self, t: f64) -> Vec3 {
        self.orig + self.dir*t
    }

    // Getters
    pub fn origin(self) -> Vec3 {
        self.orig
    }
    pub fn direction(self) -> Vec3 {
        self.dir
    }
}
