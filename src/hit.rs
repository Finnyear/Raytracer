use crate::aabb::*;
use crate::material::Material;
use crate::random::*;
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
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
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
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(Aabb {
            mn: self.center - Vec3::new(self.radius, self.radius, self.radius),
            mx: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0)))
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let oc = this_ray.ori - self.center(this_ray.tm);
        let a = this_ray.dir * this_ray.dir;
        let half_b = oc * (this_ray.dir);
        let c = oc * oc - self.radius * self.radius;
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
                let out_nor = (rec.p - self.center(this_ray.tm)) / self.radius;
                rec.set_face_normal(this_ray, out_nor);
                get_sphere_uv(out_nor, &mut rec.u, &mut rec.v);
                // rec.mat_ptr = self.mat_ptr;
                return Some(rec);
            }
            let t = (-half_b + root) / a;
            if t > tmn && t < tmx {
                rec.t = t;
                rec.p = this_ray.pos(t);
                let out_nor = (rec.p - self.center(this_ray.tm)) / self.radius;
                rec.set_face_normal(this_ray, out_nor);
                get_sphere_uv(out_nor, &mut rec.u, &mut rec.v);
                // rec.mat_ptr = self.mat_ptr;
                return Some(rec);
            }
        }
        Option::None
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let box0 = Aabb {
            mn: self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            mx: self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        };
        let box1 = Aabb {
            mn: self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            mx: self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        };
        Some(Aabb::surrounding_box(box0, box1))
    }
}
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}
impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
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
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return Option::None;
        }
        let mut output_box = Aabb {
            mn: Vec3::zero(),
            mx: Vec3::zero(),
        };
        let mut first_box = true;
        for object in self.objects.iter() {
            if let Option::Some(tmp) = object.bounding_box(t0, t1) {
                if first_box {
                    output_box = tmp.clone();
                    first_box = false;
                } else {
                    output_box = Aabb::surrounding_box(output_box, tmp);
                }
            } else {
                return Option::None;
            }
        }
        Some(output_box)
    }
}

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub mybox: Aabb,
}
impl Hittable for BvhNode {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        if self.mybox.hit(*this_ray, tmn, tmx) {
            return Option::None;
        }
        let hit_left = self.left.hit(&this_ray, tmn, tmx);
        if let Option::Some(rec_left) = hit_left {
            let hit_right = self.right.hit(&this_ray, tmn, rec_left.t);
            if let Option::Some(rec_right) = hit_right {
                return Some(rec_right);
            } else {
                return Some(rec_left);
            }
        } else {
            let hit_right = self.right.hit(&this_ray, tmn, tmx);
            if let Option::Some(rec_right) = hit_right {
                return Some(rec_right);
            } else {
                return Option::None;
            }
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(self.mybox.clone())
    }
}
impl BvhNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>, span: usize, time0: f64, time1: f64) -> Self {
        let axis = random_int(0, 2);
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if span == 1 {
            left = objects.remove(0);
            right = left.clone();
        } else if span == 2 {
            left = objects.remove(0);
            right = objects.remove(0);
        } else {
            objects.sort_by(|a, b| {
                let x = a.bounding_box(time0, time1).unwrap().mn.get(axis);
                let y = b.bounding_box(time0, time1).unwrap().mn.get(axis);
                x.partial_cmp(&y).unwrap()
            });
            let mid = span / 2;
            let (objects0, objects1) = objects.split_at_mut(mid);
            left = Arc::new(BvhNode::new(objects0.to_vec(), mid, time0, time1));
            right = Arc::new(BvhNode::new(objects1.to_vec(), span - mid, time0, time1));
        }
        let box0 = left.bounding_box(time0, time1).unwrap();
        let box1 = right.bounding_box(time0, time1).unwrap();
        Self {
            left,
            right,
            mybox: Aabb::surrounding_box(box0, box1),
        }
    }
}

pub struct xy_rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl xy_rect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl Hittable for xy_rect {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let t = (self.k - this_ray.ori.z) / this_ray.dir.z;
        if t < tmn || t > tmx {
            return Option::None;
        }
        let x = this_ray.ori.x + t * this_ray.dir.x;
        let y = this_ray.ori.y + t * this_ray.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return Option::None;
        }
        let mut rec: HitRecord = HitRecord {
            p: this_ray.pos(t),
            nor: Vec3::zero(),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            nor_dir: false,
            mat_ptr: self.mp.clone(),
        };
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(this_ray, outward_normal);
        Option::Some(rec)
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.001),
            Vec3::new(self.x1, self.y1, self.k + 0.001),
        ))
    }
}

pub struct xz_rect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl xz_rect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for xz_rect {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let t = (self.k - this_ray.ori.y) / this_ray.dir.y;
        if t < tmn || t > tmx {
            return Option::None;
        }
        let x = this_ray.ori.x + t * this_ray.dir.x;
        let z = this_ray.ori.z + t * this_ray.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }
        let mut rec: HitRecord = HitRecord {
            p: this_ray.pos(t),
            nor: Vec3::zero(),
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            nor_dir: false,
            mat_ptr: self.mp.clone(),
        };
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(this_ray, outward_normal);
        Option::Some(rec)
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.z0, self.k - 0.001),
            Vec3::new(self.x1, self.z1, self.k + 0.001),
        ))
    }
}

pub struct yz_rect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl yz_rect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for yz_rect {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let t = (self.k - this_ray.ori.x) / this_ray.dir.x;
        if t < tmn || t > tmx {
            return Option::None;
        }
        let y = this_ray.ori.y + t * this_ray.dir.y;
        let z = this_ray.ori.z + t * this_ray.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return Option::None;
        }
        let mut rec: HitRecord = HitRecord {
            p: this_ray.pos(t),
            nor: Vec3::zero(),
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            nor_dir: false,
            mat_ptr: self.mp.clone(),
        };
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(this_ray, outward_normal);
        Option::Some(rec)
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.y0, self.z0, self.k - 0.001),
            Vec3::new(self.y1, self.z1, self.k + 0.001),
        ))
    }
}
