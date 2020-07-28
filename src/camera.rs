use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}
impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = 2.0 / 1.0;
        let viewport_height = 2;
        let viewport_width = ((viewport_height as f64) * aspect_ratio) as u32;
        let focal_length = 1;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
        let lower_left_corner = origin
            - (horizontal / 2.0)
            - (vertical / 2.0)
            - Vec3::new(0.0, 0.0, focal_length as f64);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * x + self.vertical * y - self.origin,
        )
    }
}
