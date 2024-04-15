use std::default::Default;
use std::io::{self, Write};

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::utils;
use crate::color;

pub struct Camera {
    // Image
    pub image_width:  i32,
    pub image_height: i32,
    aspect_ratio: f64,

    // Pixel
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pixel_sample_scale:f64,

    // Camera things
    focal_length: f64,    // Distance to viewport
    viewport_height: f64, 
    viewport_width: f64,

    camera_center: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    viewport_upper_left: Vec3,
    pixel100_loc: Vec3,
}

impl Camera {
    pub fn from_resolution(image_width: i32, image_height: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        //let image_width = 256;
        //let image_height = 256;
        let aspect_ratio = image_width as f64 / image_height as f64;

        let pixel_sample_scale = 1.0/samples_per_pixel as f64;

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

        Self {
            image_width,
            image_height,
            aspect_ratio,

            samples_per_pixel,
            max_depth,
            pixel_sample_scale,


            focal_length,
            viewport_height,
            viewport_width,

            camera_center,
            viewport_u,
            viewport_v,

            pixel_delta_u,
            pixel_delta_v,

            viewport_upper_left,
            pixel100_loc,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let mut writer = io::stdout(); // Can be changed later to any file we want
        write!(writer, "P3\n{} {}\n255\n", self.image_width, self.image_height)
            .unwrap();

        for y in 0..self.image_height {
            eprint!("\rscanning line {}", y);
            for x in 0..self.image_width {

                let mut color = Vec3::new(0.0, 0.0, 0.0);

                for i in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    color = color + self.ray_color(r, self.max_depth, world);
                }

                color = color * self.pixel_sample_scale;

                color::color_print(&mut writer, color).unwrap();
            }
        }
        eprintln!("\rDone.                    ");
    }

    fn ray_color(&self, r: Ray, depth: i32, world: &dyn Hittable) -> Vec3 {
        if depth <= 0 { // When max depth has been reached
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(r, Interval::new(0.001, utils::INFINITY), &mut rec) {
            let direction = Vec3::rand_on_hemisphere(rec.normal);
            return 0.5 * self.ray_color(Ray::new(rec.p, direction), depth-1, world);
            //return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
        }
        
        // Calculate sky color depending on ray unit y coordinate
        let unit_direction = r.direction().unit();
        let a = 0.5*(unit_direction.y + 1.0);
        (1.0-a)*Vec3::new(1.0, 1.0, 1.0) + a*Vec3::new(0.6, 0.5, 1.0)
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {

        // Get random sample position
        let rand_offset = Self::sample_square();

        let pixel_center = self.pixel100_loc + 
            ((x as f64 + rand_offset.x)*self.pixel_delta_u) + 
            ((y as f64 + rand_offset.y)*self.pixel_delta_v);
        let ray_direction = pixel_center - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }

    // Creates a random sample 
    fn sample_square() -> Vec3 {
        Vec3::new(utils::rand_float() - 0.5, utils::rand_float() - 0.5, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::from_resolution(256, 256, 100, 10)
    }
}

