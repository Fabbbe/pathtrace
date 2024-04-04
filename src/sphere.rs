
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hittable for Sphere {

    fn hit(self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        // ( -b - sqrt(b*b - 4*a*c) ) / (2*a)
        let discriminant = half_b*half_b - a*c;

        if discriminant < 0.0 { return false; }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (-half_b + sqrt_d) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}
