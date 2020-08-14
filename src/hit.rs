use crate::aabb::*;
use crate::camera::degrees_to_radians;
use crate::material::Material;
use crate::onb::ONB;
use crate::random::*;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
pub const PI: f64 = std::f64::consts::PI;
pub const INF: f64 = std::f64::MAX;
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
    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
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
#[allow(dead_code)]
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
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb {
            mn: self.center - Vec3::new(self.radius, self.radius, self.radius),
            mx: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        if let Option::Some(_rec) = self.hit(&Ray::new(o, v, 0.0), 0.001, INF) {
            let costheta_max =
                (1.0 - self.radius * self.radius / (self.center - o).squared_length()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - costheta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let dir = self.center - o;
        let dis_squared = dir.squared_length();
        let uvw = ONB::buildw(dir);
        uvw.change(random_to_sphere(self.radius, dis_squared))
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
#[allow(dead_code)]
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
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in self.objects.iter() {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }
    fn random(&self, o: Vec3) -> Vec3 {
        self.objects[random_int(0, self.objects.len() as i32 - 1) as usize].random(o)
    }
}

pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub mybox: Aabb,
}
impl Hittable for BvhNode {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        if !self.mybox.hit(*this_ray, tmn, tmx) {
            Option::None
        } else {
            let hit_left = self.left.hit(&this_ray, tmn, tmx);
            if let Option::Some(rec_left) = hit_left {
                let hit_right = self.right.hit(&this_ray, tmn, rec_left.t);
                if let Option::Some(rec_right) = hit_right {
                    Option::Some(rec_right)
                } else {
                    Option::Some(rec_left)
                }
            } else {
                let hit_right = self.right.hit(&this_ray, tmn, tmx);
                if let Option::Some(rec_right) = hit_right {
                    Option::Some(rec_right)
                } else {
                    Option::None
                }
            }
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(self.mybox.clone())
    }
}
#[allow(dead_code)]
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

pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl XyRect {
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

impl Hittable for XyRect {
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
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.001),
            Vec3::new(self.x1, self.y1, self.k + 0.001),
        ))
    }
}

pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl XzRect {
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

impl Hittable for XzRect {
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
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(this_ray, outward_normal);
        Option::Some(rec)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.z0, self.k - 0.001),
            Vec3::new(self.x1, self.z1, self.k + 0.001),
        ))
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        if let Option::Some(rec) = self.hit(&Ray::new(o, v, 0.0), 0.001, INF) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let dis_squared = rec.t * rec.t * v.squared_length();
            let cos = ((v * rec.nor) / v.length()).abs();
            dis_squared / (cos * area)
        } else {
            0.0
        }
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Vec3::new(
            get_rand(self.x0, self.x1),
            self.k,
            get_rand(self.z0, self.z1),
        );
        random_point - o
    }
}

pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl YzRect {
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

impl Hittable for YzRect {
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
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(this_ray, outward_normal);
        Option::Some(rec)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.y0, self.z0, self.k - 0.001),
            Vec3::new(self.y1, self.z1, self.k + 0.001),
        ))
    }
}

pub struct Bbox {
    boxmn: Vec3,
    boxmx: Vec3,
    sides: HittableList,
}

impl Bbox {
    pub fn new(boxmn: Vec3, boxmx: Vec3, mat_ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::default();
        sides.add(Arc::new(XyRect::new(
            boxmn.x,
            boxmx.x,
            boxmn.y,
            boxmx.y,
            boxmn.z,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XyRect::new(
            boxmn.x,
            boxmx.x,
            boxmn.y,
            boxmx.y,
            boxmx.z,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XzRect::new(
            boxmn.x,
            boxmx.x,
            boxmn.z,
            boxmx.z,
            boxmn.y,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(XzRect::new(
            boxmn.x,
            boxmx.x,
            boxmn.z,
            boxmx.z,
            boxmx.y,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(YzRect::new(
            boxmn.y,
            boxmx.y,
            boxmn.z,
            boxmx.z,
            boxmn.x,
            mat_ptr.clone(),
        )));
        sides.add(Arc::new(YzRect::new(
            boxmn.y,
            boxmx.y,
            boxmn.z,
            boxmx.z,
            boxmx.x,
            mat_ptr.clone(),
        )));
        Self {
            boxmn,
            boxmx,
            sides,
        }
    }
}

impl Hittable for Bbox {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        self.sides.hit(this_ray, tmn, tmx)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.boxmn, self.boxmx))
    }
}

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(this_ray.ori - self.offset, this_ray.dir, this_ray.tm);
        if let Option::Some(mut rec) = self.ptr.hit(&moved_ray, tmn, tmx) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_ray, rec.nor);
            Option::Some(rec)
        } else {
            Option::None
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if let Option::Some(tmp) = self.ptr.bounding_box(t0, t1) {
            let output_box = Aabb::new(tmp.mn + self.offset, tmp.mx + self.offset);
            Option::Some(output_box)
        } else {
            Option::None
        }
    }
}

pub struct Rotatey {
    ptr: Arc<dyn Hittable>,
    sintheta: f64,
    costheta: f64,
    mybox: Option<Aabb>,
}

impl Rotatey {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radian = degrees_to_radians(angle);
        let sintheta = radian.sin();
        let costheta = radian.cos();
        let mybox = ptr.bounding_box(0.0, 1.0);
        if mybox.is_none() {
            return Self {
                ptr,
                sintheta,
                costheta,
                mybox,
            };
        }
        let tmp = mybox.unwrap();
        let mut mn = Vec3::new(INF, INF, INF);
        let mut mx = Vec3::new(-INF, -INF, -INF);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * tmp.mn.x + (1 - i) as f64 * tmp.mx.x;
                    let y = j as f64 * tmp.mn.y + (1 - j) as f64 * tmp.mx.y;
                    let z = k as f64 * tmp.mn.z + (1 - k) as f64 * tmp.mx.z;
                    let newx = x * costheta + z * sintheta;
                    let newz = -x * sintheta + z * costheta;
                    let rotated_pos = Vec3::new(newx, y, newz);

                    // mn.x = mn.x.min(rotated_pos.x);
                    // mx.x = mx.x.max(rotated_pos.x);

                    // mn.y = mn.y.min(rotated_pos.y);
                    // mx.y = mx.y.max(rotated_pos.y);

                    // mn.z = mn.z.min(rotated_pos.z);
                    // mx.z = mx.z.max(rotated_pos.z);
                    for c in 0..3 {
                        *mn.get_mut(c) = mn.get(c).min(rotated_pos.get(c));
                        *mx.get_mut(c) = mx.get(c).max(rotated_pos.get(c));
                    }
                }
            }
        }
        let mybox = Option::Some(Aabb::new(mn, mx));
        Self {
            ptr,
            sintheta,
            costheta,
            mybox,
        }
    }
}

impl Hittable for Rotatey {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        let mut ori = this_ray.ori;
        let mut dir = this_ray.dir;
        ori.x = self.costheta * this_ray.ori.x - self.sintheta * this_ray.ori.z;
        ori.z = self.sintheta * this_ray.ori.x + self.costheta * this_ray.ori.z;
        dir.x = self.costheta * this_ray.dir.x - self.sintheta * this_ray.dir.z;
        dir.z = self.sintheta * this_ray.dir.x + self.costheta * this_ray.dir.z;
        let rotated_ray = Ray::new(ori, dir, this_ray.tm);
        if let Option::Some(mut rec) = self.ptr.hit(&rotated_ray, tmn, tmx) {
            let mut p = rec.p;
            let mut nor = rec.nor;
            p.x = self.costheta * rec.p.x + self.sintheta * rec.p.z;
            p.z = -self.sintheta * rec.p.x + self.costheta * rec.p.z;
            nor.x = self.costheta * rec.nor.x + self.sintheta * rec.nor.z;
            nor.z = -self.sintheta * rec.nor.x + self.costheta * rec.nor.z;
            rec.p = p;
            rec.set_face_normal(&rotated_ray, nor);
            Option::Some(rec)
        } else {
            Option::None
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        self.mybox.clone()
    }
}

pub struct FlipFace {
    ptr: Arc<dyn Hittable>,
}
impl FlipFace {
    pub fn new(ptr: Arc<dyn Hittable>) -> Self {
        Self { ptr }
    }
}
impl Hittable for FlipFace {
    fn hit(&self, this_ray: &Ray, tmn: f64, tmx: f64) -> Option<HitRecord> {
        if let Option::Some(mut rec) = self.ptr.hit(this_ray, tmn, tmx) {
            rec.nor_dir = !rec.nor_dir;
            Option::Some(rec)
        } else {
            Option::None
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(t0, t1)
    }
}
