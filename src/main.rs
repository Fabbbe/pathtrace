use std::io::{self, Write};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;

use vec3::Vec3;
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(r.direction());
    let c = oc.length_squared() - radius*radius;

    // ( -b - sqrt(b*b - 4*a*c) ) / (2*a)
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 {
        return -1.0;
    }
    (-half_b - discriminant.sqrt()) / a
}

fn ray_color(r: ray::Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.4, r);

    if t > 0.0 {
        let from_center = r.at(t) - Vec3::new(0.0, 0.0, -1.0);
        return (from_center.unit() + Vec3::new(1.0, 1.0, 1.0)) / 2.0;
    }
    
    // Calculate sky color depending on ray unit y coordinate
    let unit_direction = r.direction().unit();
    let a = 0.5*(unit_direction.y + 1.0);
    (1.0-a)*Vec3::new(1.0, 1.0, 1.0) + a*Vec3::new(0.6, 0.5, 1.0)
}

fn main() {
    let image_width =  256;
    let image_height = 256;

    let aspect_ratio: f64 = image_width as f64 / image_height as f64;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
                            - Vec3::new(0.0, 0.0, focal_length)
                            - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut writer = io::stdout(); // Can be changed later to any file we want
    write!(writer, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for y in 0..image_height {
        eprint!("\rscanning line {}", y);
        for x in 0..image_width {
            let pixel_center = pixel100_loc + (x as f64*pixel_delta_u) + (y as f64*pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let color = ray_color(Ray::new(camera_center, ray_direction));

            color::color_print(&mut writer, color).unwrap();
        }
    }
    eprintln!("\rDone.                    ");
}
