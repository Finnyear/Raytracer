use crate::hit::HitRecord;
use crate::onb::ONB;
use crate::random::*;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::*;
use std::sync::Arc;
pub trait Material {
    fn scatter(&self, this_ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray, f64)> {
        Option::None
    }
    fn scattering_pdf(&self, this_ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
    fn emitted(&self, rec: &HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn newarc(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, this_ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray, f64)> {
        let uvw = ONB::buildw(rec.nor);
        let sca_dir = uvw.change(random_cosine_direction());
        let scattered = Ray {
            ori: rec.p,
            dir: sca_dir.unit(),
            tm: this_ray.tm,
        };
        let atten_col = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = (uvw.w() * scattered.dir) / PI;
        Option::Some((atten_col, scattered, pdf))
    }
    fn scattering_pdf(&self, this_ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.nor * scattered.dir.unit();
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
        //0.5 / PI
    }
}
/*
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        if fuzz > 1.0 {
            Self { albedo, fuzz: 1.0 }
        } else {
            Self { albedo, fuzz }
        }
    }
}
impl Material for Metal {
    fn scatter(&self, this_ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(this_ray.dir.unit(), rec.nor);
        let scattered = Ray {
            ori: rec.p,
            dir: reflected + random_in_unit_sphere() * self.fuzz,
            tm: 0.0,
        };
        let atten_col = self.albedo;
        if scattered.dir * rec.nor > 0.0 {
            Option::Some((atten_col, scattered))
        } else {
            Option::None
        }
    }
}
pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
pub struct Dielectric {
    ref_idx: f64,
}
impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(&self, this_ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let atten_col = Vec3::ones();
        let etai_over_etat: f64 = if rec.nor_dir {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_dir = this_ray.dir.unit();
        let cos_theta = ((-unit_dir) * rec.nor).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_dir, rec.nor);
            let scattered = Ray {
                ori: rec.p,
                dir: reflected,
                tm: 0.0,
            };
            return Option::Some((atten_col, scattered));
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if get_rand(0.0, 1.0) < reflect_prob {
            let reflected = reflect(unit_dir, rec.nor);
            let scattered = Ray {
                ori: rec.p,
                dir: reflected,
                tm: 0.0,
            };
            return Option::Some((atten_col, scattered));
        }
        let refracted = refract(unit_dir, rec.nor, etai_over_etat);
        let scattered = Ray {
            ori: rec.p,
            dir: refracted,
            tm: 0.0,
        };
        Option::Some((atten_col, scattered))
    }
}
*/
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: Vec3) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(emit)),
        }
    }
    pub fn newarc(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}
impl Material for DiffuseLight {
    /*fn scatter(&self, _this_ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        Option::None
    }*/
    fn emitted(&self, rec: &HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        if rec.nor_dir {
            self.emit.value(u, v, p)
        } else {
            Vec3::zero()
        }
    }
}
