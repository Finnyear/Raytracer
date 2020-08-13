use crate::vec3::*;
use std::sync::Arc;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct SolidColor {
    pub col: Vec3,
}
impl SolidColor {
    pub fn new(col: Vec3) -> Self {
        Self { col }
    }
    // fn new(r: f64, g: f64, b: f64) -> Self{
    // 	Self{Vec3 :: new(r, g, b)}
    // }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.col
    }
}
pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new(odd: Vec3, even: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
    pub fn newarc(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = ((p.x * 10.0).sin()) * ((p.y * 10.0).sin()) * ((p.z * 10.0).sin());
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
