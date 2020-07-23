#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
// use std;
pub use vec3::Vec3;

mod point;
pub use point::Point;
mod clock;
pub use clock::Clock;

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 512);
    let bar = ProgressBar::new(1024);
    // let mut a: [[i32; 512]; 1024] = [[0; 512]; 1024];
    // a[0][0] = 0;
    // draw_clock(&mut a);
    let mut clo = Clock :: new();
    clo.draw_clock_plate();
    clo.draw_clock_needle(19, 15, 0);
    for x in 0..1024 {
        for y in 0..512 {
            let pixel = img.get_pixel_mut(x, y);
            let color_r = (x / 4) as u8;
            let color_g = (y / 2) as u8;
            let color_b = 63;
            if clo.clock[x as usize][y as usize] == 1{
                *pixel = image::Rgb([0, 0, 0]);
            }
            else{
                *pixel = image::Rgb([255, 255, 255]);
            }
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
