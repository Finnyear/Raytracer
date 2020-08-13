use crate::vec3::Vec3;

pub struct ONB {
    pub axis: [Vec3; 3],
}
impl ONB {
    pub fn buildw(n: Vec3) -> Self {
        let axis2 = n.unit();
        let a = if axis2.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let axis1 = Vec3::cross(axis2, a);
        let axis0 = Vec3::cross(axis1, axis2);
        Self {
            axis: [axis0, axis1, axis2],
        }
    }
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.axis[0] * a + self.axis[1] * b + self.axis[2] * c
    }
    pub fn change(&self, a: Vec3) -> Vec3 {
        self.axis[0] * a.x + self.axis[1] * a.y + self.axis[2] * a.z
    }
    pub fn get(&self, a: usize) -> Vec3 {
        self.axis[a]
    }
}
