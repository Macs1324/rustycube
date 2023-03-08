use nalgebra_glm as glm;
#[derive(Clone, Copy)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl<T: Into<usize>> std::ops::Index<T> for XYZ {
    type Output = f32;
    fn index(&self, index: T) -> &Self::Output {
        match index.into() {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,

            _ => panic!("Indexing XYZ out of bounds!"),
        }
    }
}

impl Into<glm::Vec3> for XYZ {
    fn into(self) -> glm::Vec3 {
        glm::Vec3::new(self.x, self.y, self.z)
    }
}
