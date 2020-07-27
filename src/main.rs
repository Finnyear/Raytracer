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
pub const PI: f64 = std::f64::consts::PI;
use rand::prelude::*;


pub fn get_rand(mn:f64, mx:f64) -> f64{
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen(); // generates a float between 0 and 1
    x * (mx - mn) + mn
}
pub fn get_rand_vec3(mn:f64, mx:f64) -> Vec3{
    Vec3 :: new(get_rand(mn, mx), get_rand(mn, mx), get_rand(mn, mx))
}
pub fn random_unit_vector() -> Vec3{
    let a = get_rand(0.0, 2.0 * PI);
    let z = get_rand(-1.0, 1.0);
    let r = (1.0 - z*z).sqrt();
    return Vec3 :: new(r * a.cos(), r * a.sin(), z);
}
pub fn random_in_unit_sphere() -> Vec3{
    loop{
        let p = get_rand_vec3(-1.0, 1.0);
        if p.length() < 1.0 {return p}
    }
}
pub fn random_in_hemisphere(nor : Vec3) -> Vec3{
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere * nor > 0.0 {in_unit_sphere}
    else{-in_unit_sphere}
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

fn get_color(this_ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0{
        return Vec3 :: zero();
    }
    if let Option::Some(rec) = world.hit(this_ray, 0.001, INF) {
        let target = rec.p + random_in_hemisphere(rec.nor);
        return get_color(&Ray :: new(rec.p, target - rec.p), world, depth - 1) * 0.5;
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
    const SAM_NUM: i32 = 10;
    const MAX_DEP: i32 = 5;

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
            for _i in 0..SAM_NUM{
                let dx = (x as f64 + get_rand(0.0, 1.0)) / (image_width as f64);
                let dy = (y as f64 + get_rand(0.0, 1.0)) / (image_height as f64);
                let this_ray = cam.get_ray(dx, dy);
                color = color + get_color(&this_ray, &world, MAX_DEP);
            }
            *pixel = Rgb([
                ((color.x / SAM_NUM as f64).sqrt() * 255.0) as u8,
                ((color.y / SAM_NUM as f64).sqrt() * 255.0) as u8,
                ((color.z / SAM_NUM as f64).sqrt() * 255.0) as u8,
                // (color.x as f64 * 255.0) as u8,
                // (color.y as f64 * 255.0) as u8,
                // (color.z as f64 * 255.0) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
