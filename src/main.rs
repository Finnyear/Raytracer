#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
// use std;
pub use vec3::Vec3;
mod ray;
pub use image::Rgb;
pub use ray::Ray;

fn get_background(this_ray: Ray) -> Vec3 {
    let unit_dir = this_ray.dir().unit();
    let t: f64 = (unit_dir.y() + 1.0) / 2.0;
    (Vec3::new(1.0, 1.0, 1.0) * t) + (Vec3::new(0.5, 0.7, 1.0) * (1.0 - t))
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let aspect_ratio: f64 = 2.0 / 1.0;
    let image_height = 512;
    let image_width = ((image_height as f64) * aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(image_width as u64);

    let viewport_height = 2;
    let viewport_width = ((viewport_height as f64) * aspect_ratio) as u32;
    let focal_length = 1;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
    let upper_left_corner =
        origin - (horizontal / 2.0) + (vertical / 2.0) + Vec3::new(0.0, 0.0, focal_length as f64);

    for x in 0..image_width {
        for y in 0..image_height {
            let pixel = img.get_pixel_mut(x, y);
            let dx = (x as f64) / (image_width as f64);
            let dy = (y as f64) / (image_height as f64);
            let this_ray = Ray::new(
                origin,
                upper_left_corner + horizontal * dx + vertical * dy - origin,
            );
            let background = get_background(this_ray);
            *pixel = Rgb([
                (background.x() * 255.0) as u8,
                (background.y() * 255.0) as u8,
                (background.z() * 255.0) as u8,
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
