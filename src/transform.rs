use crate::xyz::XYZ;
use nalgebra_glm as glm;

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: XYZ,
    rotation: XYZ,
    pub scale: XYZ,
}

impl Transform {
    pub fn zero() -> Transform {
        Transform {
            position: XYZ {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            rotation: XYZ {
                x: 0f32,
                y: 0f32,
                z: 0f32,
            },
            scale: XYZ {
                x: 1f32,
                y: 1f32,
                z: 1f32,
            },
        }
    }
    pub fn to_matrix(&self) -> glm::Mat4 {
        let mut r = glm::identity();

        r = glm::scale(&r, &self.scale.into());

        r = glm::rotate(&r, self.rotation.x, &glm::Vec3::x());
        r = glm::rotate(&r, self.rotation.y, &glm::Vec3::y());
        r = glm::rotate(&r, self.rotation.z, &glm::Vec3::z());

        r = glm::translate(&r, &self.position.into());

        r
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation.x += angle % 360f32;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation.y += angle % 360f32;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation.z += angle % 360f32;
    }

    pub fn get_rotation(&self) -> XYZ {
        self.rotation
    }

    pub fn set_rotation_x(&mut self, angle: f32) {
        self.rotation.x = angle % 360f32;
    }

    pub fn set_rotation_y(&mut self, angle: f32) {
        self.rotation.y = angle % 360f32;
    }

    pub fn set_rotation_z(&mut self, angle: f32) {
        self.rotation.z = angle % 360f32;
    }
}
pub fn mat2array(m: glm::Mat4) -> [[f32; 4]; 4] {
    let mut r: [[f32; 4]; 4] = [[0f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            r[i][j] = m[i * 4 + j]
        }
    }

    r
}
