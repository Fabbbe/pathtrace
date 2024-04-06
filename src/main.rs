mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod utils;
mod interval;
mod camera;

use vec3::Vec3;
use hittable::list::HittableList;
use sphere::Sphere;
use camera::Camera;

fn main() {
    // World:
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.4)));
    world.add(Box::new(Sphere::new(Vec3::new(0.4, 0.5, -1.3), 0.2)));
    //world.add(Box::new(Sphere::new(Vec3::new(0.0, -101.0, -1.0), 100.0)));

    // Camera
    let cam = Camera::from_resolution(256, 256);

    cam.render(&world);
}

