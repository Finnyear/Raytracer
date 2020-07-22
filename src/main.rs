#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let color_r = (x / 4) as u8;
            let color_g = (y / 2) as u8;
            // let mut color_b = 0;
            // if x / 4 + y / 2 < 256 {
            //     color_b = (0xff - x / 4 - y / 2) as u8;
            // }
            let color_b = 63;
            *pixel = image::Rgb([color_r, color_g, color_b]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
