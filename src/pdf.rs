use crate::hit::*;
use crate::onb::ONB;
use crate::random::*;
use crate::vec3::Vec3;
pub const PI: f64 = std::f64::consts::PI;
use std::sync::Arc;

pub trait PDF {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
pub struct CosPDF {
    pub uvw: ONB,
}
impl CosPDF {
    pub fn new(w: Vec3) -> Self {
        let uvw = ONB::buildw(w);
        Self { uvw }
    }
}
impl PDF for CosPDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cos = direction.unit() * self.uvw.w();
        if cos <= 0.0 {
            0.0
        } else {
            cos / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.change(random_cosine_direction())
    }
}

pub struct HittablePDF {
    pub ptr: Arc<dyn Hittable>,
    pub o: Vec3,
}
impl HittablePDF {
    pub fn new(ptr: Arc<dyn Hittable>, o: Vec3) -> Self {
        Self { ptr, o }
    }
}
impl PDF for HittablePDF {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}

pub struct MixturePDF {
    pub p: [Arc<dyn PDF>; 2],
}
impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self { p: [p0, p1] }
    }
}
impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if get_rand01() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}

pub struct NOPDF {}
impl PDF for NOPDF {
    fn value(&self, _direction: Vec3) -> f64 {
        unreachable!()
    }
    fn generate(&self) -> Vec3 {
        unreachable!()
    }
}
