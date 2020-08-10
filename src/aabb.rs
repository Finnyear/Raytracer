use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Aabb {
    pub mn: Vec3,
    pub mx: Vec3,
}
impl Aabb {
    pub fn new(mn: Vec3, mx: Vec3) -> Self {
        Self { mn, mx }
    }
    pub fn hit(&self, this_ray: Ray, tmin: f64, tmax: f64) -> bool {
        for i in 0..3 {
            let inv = 1.0 / this_ray.dir.get(i);
            let mut t0 = (self.mn.get(i) - this_ray.ori.get(i)) * inv;
            let mut t1 = (self.mx.get(i) - this_ray.ori.get(i)) * inv;
            if inv < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > tmin {
                let tmin = t0;
            }
            if t1 < tmax {
                let tmax = t1;
            }
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        let _mn = Vec3::new(
            box0.mn.x.min(box1.mn.x),
            box0.mn.y.min(box1.mn.y),
            box0.mn.z.min(box1.mn.z),
        );
        let _mx = Vec3::new(
            box0.mx.x.max(box1.mx.x),
            box0.mx.y.max(box1.mx.y),
            box0.mx.z.max(box1.mx.z),
        );
        Aabb { mn: _mn, mx: _mx }
    }
}
