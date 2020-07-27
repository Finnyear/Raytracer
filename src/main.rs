#[allow(clippy::float_cmp)]
extern crate rand;
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
// use std;
pub use vec3::Vec3;
mod ray;
pub use image::Rgb;
pub use ray::Ray;
mod hit;
use hit::*;
mod camera;
use camera::Camera;
pub const INF: f64 = std::f64::INFINITY;
use rand::prelude::*;


pub fn get_rand(low:f64, high:f64) -> f64{
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen(); // generates a float between 0 and 1
    x * (high - low) + low
}

/*fn hit_sphere(center: Vec3, radius: f64, this_ray: &Ray) -> f64 {
    let a = this_ray.dir * this_ray.dir;
    let half_b = (this_ray.ori - center) * (this_ray.dir);
    let c = (this_ray.ori - center) * (this_ray.ori - center) - radius * radius;
    let dt = half_b * half_b - a * c;
    if dt <= 0.0 {
        -1.0
    } else {
        (-half_b - dt.sqrt()) / a
    }
}*/

fn get_color(this_ray: &Ray, world: &HittableList) -> Vec3 {
    if let Option::Some(rec) = world.hit(this_ray, 0.0, INF) {
        return Vec3::new(rec.nor.x + 1.0, rec.nor.y + 1.0, rec.nor.z + 1.0) * 0.5;
    }
    let unit_dir = this_ray.dir.unit();
    let k: f64 = (unit_dir.y + 1.0) / 2.0;
    (Vec3::new(1.0, 1.0, 1.0) * k) + (Vec3::new(0.5, 0.7, 1.0) * (1.0 - k))
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let aspect_ratio: f64 = 2.0 / 1.0;
    let image_height = 512;
    let image_width = ((image_height as f64) * aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    const samples_per_pixel: i32 = 100;

    // let viewport_height = 2;
    // let viewport_width = ((viewport_height as f64) * aspect_ratio) as u32;
    // let focal_length = 1;

    // let origin = Vec3::new(0.0, 0.0, 0.0);
    // let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
    // let lower_left_corner =
    //     origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length as f64);

    let mut world: HittableList = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam: Camera = Camera :: new();

    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, image_height - 1 - y);
            let mut color:Vec3 = Vec3 :: zero();
            for i in 0..samples_per_pixel{
                let dx = (x as f64 + get_rand(0.0, 1.0)) / (image_width as f64);
                let dy = (y as f64 + get_rand(0.0, 1.0)) / (image_height as f64);
                let this_ray = cam.get_ray(dx, dy);
                color = color + get_color(&this_ray, &world) / samples_per_pixel as f64 ;
            }
            *pixel = Rgb([
                (color.x as f64 * 255.0) as u8,
                (color.y as f64 * 255.0) as u8,
                (color.z as f64 * 255.0) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
