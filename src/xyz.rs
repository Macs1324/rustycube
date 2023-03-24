use std::ops::{Add, Sub};

use nalgebra_glm as glm;
#[derive(Clone, Copy, Debug)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZ {
    pub fn rad(&self) -> XYZ {
        XYZ {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
            z: self.z.to_radians(),
        }
    }

    pub fn deg(&self) -> XYZ {
        XYZ {
            x: self.x.to_degrees(),
            y: self.y.to_degrees(),
            z: self.z.to_degrees(),
        }
    }
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

impl Add for XYZ {
    type Output = XYZ;
    fn add(self, other: XYZ) -> XYZ {
        XYZ {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for XYZ {
    type Output = XYZ;
    fn sub(self, other: XYZ) -> XYZ {
        XYZ {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
