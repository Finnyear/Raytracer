use crate::random::*;
use crate::ray::Ray;
use crate::vec3::*;
pub const PI: f64 = std::f64::consts::PI;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}
impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = u * (focus_dist * viewport_width);
        let vertical = v * (focus_dist * viewport_height);
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_dist;
        let lens_radius = aperture / 2.0;
        // let focal_length = 1.0;

        // let origin = Vec3::new(0.0, 0.0, 0.0);
        // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        // let vertical = Vec3::new(0.0, viewport_height, 0.0);
        // let lower_left_corner =
        //     origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1,
        }
    }
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * x + self.vertical * y - self.origin - offset,
            get_rand(self.time0, self.time1),
        )
    }
}
