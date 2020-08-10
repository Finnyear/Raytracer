extern crate rand;
use crate::vec3::Vec3;
use rand::prelude::*;
pub const PI: f64 = std::f64::consts::PI;
pub fn get_rand01() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen() // generates a float between 0 and 1
}
pub fn get_rand(mn: f64, mx: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen(); // generates a float between 0 and 1
    x * (mx - mn) + mn
}
pub fn random_int(mn: i32, mx: i32) -> i32 {
    get_rand(mn as f64, mx as f64 + 1.0) as i32
}
pub fn get_rand_vec3(mn: f64, mx: f64) -> Vec3 {
    Vec3::new(get_rand(mn, mx), get_rand(mn, mx), get_rand(mn, mx))
}
pub fn random_unit_vector() -> Vec3 {
    let a = get_rand(0.0, 2.0 * PI);
    let z = get_rand(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = get_rand_vec3(-1.0, 1.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}
#[allow(dead_code)]
pub fn random_in_hemisphere(nor: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere * nor > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
