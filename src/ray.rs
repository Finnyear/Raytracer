use crate::vec3::Vec3;
pub struct Ray {
    ori: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(ori: Vec3, dir: Vec3) -> Self {
        Self { ori, dir }
    }
    pub fn pos(&self, t: f64) -> Vec3 {
        self.ori + (self.dir * t)
    }
    pub fn ori(&self) -> Vec3 {
        self.ori
    }
    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}
