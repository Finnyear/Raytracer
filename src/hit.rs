use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
pub const PI: f64 = std::f64::consts::PI;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub nor: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub nor_dir: bool,
    pub mat_ptr: Arc<dyn Material>,
}
impl HitRecord {
    fn set_face_normal(&mut self, this_ray: &Ray, out_nor: Vec3) {
        self.nor_dir = this_ray.dir * out_nor < 0.0;
        if self.nor_dir {
            self.nor = out_nor;
        } else {
            self.nor = -out_nor;
        }
    }
}
pub trait Hittable {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord>;
}
pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    *u = 1.0 - (phi + PI) / (2.0 * PI);
    *v = (theta + PI / 2.0) / PI;
}
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let a = this_ray.dir * this_ray.dir;
        let half_b = (this_ray.ori - self.center) * (this_ray.dir);
        let c =
            (this_ray.ori - self.center) * (this_ray.ori - self.center) - self.radius * self.radius;
        let dt = half_b * half_b - a * c;
        let mut rec: HitRecord = HitRecord {
            p: Vec3::zero(),
            nor: Vec3::zero(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            nor_dir: false,
            mat_ptr: self.mat_ptr.clone(),
        };
        if dt > 0.0 {
            let root = dt.sqrt();
            let t = (-half_b - root) / a;
            if t > tmn && t < tmx {
                rec.t = t;
                rec.p = this_ray.pos(t);
                let out_nor = (rec.p - self.center) / self.radius;
                rec.set_face_normal(this_ray, out_nor);
                get_sphere_uv((rec.p - self.center) / self.radius, &mut rec.u, &mut rec.v);
                // rec.mat_ptr = self.mat_ptr;
                return Some(rec);
            }
            let t = (-half_b + root) / a;
            if t > tmn && t < tmx {
                rec.t = t;
                rec.p = this_ray.pos(t);
                let out_nor = (rec.p - self.center) / self.radius;
                rec.set_face_normal(this_ray, out_nor);
                get_sphere_uv((rec.p - self.center) / self.radius, &mut rec.u, &mut rec.v);
                // rec.mat_ptr = self.mat_ptr;
                return Some(rec);
            }
        }
        Option::None
    }
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
impl Hittable for HittableList {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = Option::None;
        let mut _tmx = tmx;
        for object in self.objects.iter() {
            if let Option::Some(_rec) = object.hit(this_ray, tmn, _tmx) {
                rec = Option::Some(_rec.clone());
                _tmx = _rec.t;
            }
        }
        rec
    }
}
