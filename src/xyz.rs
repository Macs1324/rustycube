use std::ops::{Add, Sub};

use nalgebra_glm as glm;
#[derive(Clone, Copy, Debug)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZ {
    pub fn zero() -> XYZ {
        XYZ {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
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

    pub fn length(&self) -> f32 {
        (self.x * self.x * self.x + self.y * self.y * self.y + self.z * self.z * self.z).cbrt()
    }

    pub fn normalized(&self) -> XYZ {
        let length = self.length();
        if length == 0.0 {
            return XYZ::zero();
        }
        *self / length
    }

    pub fn rotated_y(&self, angle: f32) -> XYZ {
        XYZ {
            x: self.x * angle.cos() - self.z * angle.sin(),
            y: self.y,
            z: self.x * angle.sin() + self.z * angle.cos(),
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

impl Into<[f32; 3]> for XYZ {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
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

impl Add<f32> for XYZ {
    type Output = XYZ;
    fn add(self, other: f32) -> XYZ {
        XYZ {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub<f32> for XYZ {
    type Output = XYZ;
    fn sub(self, other: f32) -> XYZ {
        XYZ {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl std::ops::Mul<f32> for XYZ {
    type Output = XYZ;
    fn mul(self, other: f32) -> XYZ {
        XYZ {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Div<f32> for XYZ {
    type Output = XYZ;
    fn div(self, other: f32) -> XYZ {
        XYZ {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
