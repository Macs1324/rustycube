use crate::{transform::Transform, vector3::Vector3};
use nalgebra_glm as glm;

pub trait Camera {
    fn projection(&self) -> glm::Mat4;
    fn view(&self, transform: Transform) -> glm::Mat4;
}

pub struct NoCamera {}

impl Camera for NoCamera {
    fn projection(&self) -> glm::Mat4 {
        glm::identity()
    }

    fn view(&self, _transform: Transform) -> glm::Mat4 {
        glm::identity()
    }
}
pub struct PerspectiveCamera3D {
    pub aspect: f32,
    pub fov: f32,
    pub clip_near: f32,
    pub clip_far: f32,
}

impl PerspectiveCamera3D {
    pub fn new(aspect: f32, fov: f32, near: f32, far: f32) -> PerspectiveCamera3D {
        PerspectiveCamera3D {
            aspect,
            fov,
            clip_near: near,
            clip_far: far,
        }
    }
}

impl Camera for PerspectiveCamera3D {
    fn projection(&self) -> glm::Mat4 {
        glm::perspective(self.aspect, self.fov, self.clip_near, self.clip_far)
    }
    fn view(&self, transform: Transform) -> glm::Mat4 {
        let mut lookat_point = glm::Vec3::zeros();
        lookat_point.x = transform.get_rotation().y.cos() * transform.get_rotation().x.cos();
        lookat_point.y = transform.get_rotation().x.sin();
        lookat_point.z = transform.get_rotation().y.sin() * transform.get_rotation().x.cos();

        lookat_point += &transform.position.into();
        glm::look_at_rh(&transform.position.into(), &lookat_point, &glm::Vec3::y())
    }
}
