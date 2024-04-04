
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,

    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector

        self.front_face = r.direction().dot(outward_normal) > 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

