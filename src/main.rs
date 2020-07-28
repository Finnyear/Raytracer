#![allow(clippy::float_cmp)]
pub use image::Rgb;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;
// use std;
mod vec3;
pub use vec3::*;
mod ray;
pub use ray::Ray;
mod hit;
use hit::*;
mod camera;
use camera::Camera;
mod material;
use material::*;
pub const INF: f64 = std::f64::MAX;
pub const PI: f64 = std::f64::consts::PI;
mod random;
use random::*;

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
// pub fn degrees_to_radians(degrees: f64) -> f64 {
//     degrees * PI / 180.0
// }
fn get_color(this_ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    if let Option::Some(rec) = world.hit(this_ray, 0.001, INF) {
        // let target = rec.p + random_in_hemisphere(rec.nor);
        if let Option::Some((atten_col, scattered)) = rec.mat_ptr.scatter(this_ray, &rec) {
            return get_color(&scattered, world, depth - 1).change(atten_col);
        }
        return Vec3::zero();
    }
    let unit_dir = this_ray.dir.unit();
    let k: f64 = (unit_dir.y + 1.0) / 2.0;
    (Vec3::new(1.0, 1.0, 1.0) * (1.0 - k)) + (Vec3::new(0.5, 0.7, 1.0) * k)
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let mat_ground = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground.clone(),
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = get_rand01();
            let center = Vec3::new(
                i as f64 + 0.9 * get_rand01(),
                0.2,
                j as f64 + 0.9 * get_rand01(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if (choose_mat < 0.8) {
                    let albedo = Vec3::random01().change(Vec3::random01());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = get_rand(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let aspect_ratio: f64 = 2.0 / 1.0;
    let image_height: u32 = 512;
    let image_width: u32 = ((image_height as f64) * aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    let SAM_NUM: i32 = 100;
    let MAX_DEP: i32 = 50;

    // let viewport_height = 2;
    // let viewport_width = ((viewport_height as f64) * aspect_ratio) as u32;
    // let focal_length = 1;

    // let origin = Vec3::new(0.0, 0.0, 0.0);
    // let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
    // let lower_left_corner =
    //     origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length as f64);

    // let R = (PI / 4.0).cos();

    // let mat_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    // let mat_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    // let mat_left = Arc::new(Dielectric::new(1.5));
    // let mat_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));
    // let mat_left = Arc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    // let mat_right = Arc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));

    let world: HittableList = random_scene();
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(-R, 0.0, -1.0),
    //     R,
    //     mat_left.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(R, 0.0, -1.0),
    //     R,
    //     mat_right.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     mat_ground.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     mat_center.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     mat_left.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     -0.4,
    //     mat_left.clone(),
    // )));
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     mat_right.clone(),
    // )));

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    reflect(Vec3::ones(), Vec3::ones());

    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, image_height - 1 - y);
            let mut color: Vec3 = Vec3::zero();
            for _i in 0..SAM_NUM {
                let dx = (x as f64 + get_rand01()) / (image_width as f64);
                let dy = (y as f64 + get_rand01()) / (image_height as f64);
                let this_ray = cam.get_ray(dx, dy);
                color += get_color(&this_ray, &world, MAX_DEP);
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
