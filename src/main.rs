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
mod texture;
use texture::*;
mod aabb;

// fn get_color(this_ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
//     if depth <= 0 {
//         return Vec3::zero();
//     }
//     if let Option::Some(rec) = world.hit(this_ray, 0.001, INF) {
//         // let target = rec.p + random_in_hemisphere(rec.nor);
//         if let Option::Some((atten_col, scattered)) = rec.mat_ptr.scatter(this_ray, &rec) {
//             return get_color(&scattered, world, depth - 1).change(atten_col);
//         }
//         return Vec3::zero();
//     }
//     let unit_dir = this_ray.dir.unit();
//     let k: f64 = (unit_dir.y + 1.0) / 2.0;
//     (Vec3::new(1.0, 1.0, 1.0) * (1.0 - k)) + (Vec3::new(0.5, 0.7, 1.0) * k)
// }

fn get_color(this_ray: &Ray, background: Vec3, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    if let Option::Some(rec) = world.hit(this_ray, 0.001, INF) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        if let Option::Some((atten_col, scattered)) = rec.mat_ptr.scatter(this_ray, &rec) {
            return emitted + get_color(&scattered, background, world, depth - 1).change(atten_col);
        }
        emitted
    } else {
        background
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::newarc(checker)),
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
                if choose_mat < 0.8 {
                    let albedo = Vec3::random01().change(Vec3::random01());
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, get_rand(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = get_rand(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::default();
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    objects
}

pub fn simple_light() -> HittableList {
    let mut objects = HittableList::default();
    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::newarc(checker.clone())),
    )));
    let difflight = Arc::new(DiffuseLight::new(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}

pub fn cornellbox() -> HittableList {
    let mut objects = HittableList::default();

    let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Vec3::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let ns = 100;
    let mut boxes0 = HittableList::default();
    for i in 0..ns {
        boxes0.add(Arc::new(Sphere::new(
            Vec3::random(165.0, 330.0),
            10.0,
            white.clone(),
        )));
    }
    objects.add(Arc::new(BvhNode::new(boxes0.objects, ns, 0.0, 1.0)));
    // let box1 = Arc::new(Bbox::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(165.0, 330.0, 165.0),
    //     white.clone(),
    // ));
    // let box1 = Arc::new(Rotatey::new(box1, 15.0));
    // let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    // objects.add(box1);

    // let box2 = Arc::new(Bbox::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     Vec3::new(165.0, 165.0, 165.0),
    //     white.clone(),
    // ));
    // let box2 = Arc::new(Rotatey::new(box2, -18.0));
    // let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    // objects.add(box2);

    return objects;
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut aspect_ratio: f64 = 2.0 / 1.0;
    let mut image_height: u32 = 512;
    let mut sam_num: i32 = 10;
    let mut max_dep: i32 = 50;

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let mut world = HittableList::default();
    let mut lookfrom: Vec3;
    let mut lookat: Vec3;
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Vec3::zero();

    // {
    //     //Case 1:
    //     world = random_scene();
    //     background = Vec3 :: new(0.70, 0.80, 1.00);
    //     lookfrom = Vec3::new(13.0, 2.0, 3.0);
    //     lookat = Vec3::new(0.0, 0.0, 0.0);
    //     vfov = 20.0;
    //     aperture = 0.1;
    // }
    // {
    //     //Case 2:
    //     world = two_spheres();
    //     background = Vec3 :: new(0.70, 0.80, 1.00);
    //     lookfrom = Vec3::new(13.0, 2.0, 3.0);
    //     lookat = Vec3::new(0.0, 0.0, 0.0);
    //     vfov = 20.0;
    // }
    // {
    //     //Case 5:
    //     world = simple_light();
    //     sam_num = 400;
    //     background = Vec3::zero();
    //     lookfrom = Vec3::new(26.0, 3.0, 6.0);
    //     lookat = Vec3::new(0.0, 2.0, 0.0);
    //     vfov = 20.0;
    // }
    {
        //Case 6:
        world = cornellbox();
        aspect_ratio = 1.0;
        image_height = 600;
        sam_num = 100;
        background = Vec3::zero();
        lookfrom = Vec3::new(278.0, 278.0, -800.0);
        lookat = Vec3::new(278.0, 278.0, 0.0);
        vfov = 40.0;
    }
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let mut image_width: u32 = ((image_height as f64) * aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);
    // reflect(Vec3::ones(), Vec3::ones());

    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, image_height - 1 - y);
            let mut color: Vec3 = Vec3::zero();
            for _i in 0..sam_num {
                let dx = (x as f64 + get_rand01()) / (image_width as f64);
                let dy = (y as f64 + get_rand01()) / (image_height as f64);
                let this_ray = cam.get_ray(dx, dy);
                color += get_color(&this_ray, background, &world, max_dep);
            }
            *pixel = Rgb([
                ((color.x / sam_num as f64).sqrt() * 255.0) as u8,
                ((color.y / sam_num as f64).sqrt() * 255.0) as u8,
                ((color.z / sam_num as f64).sqrt() * 255.0) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
