use crate::xyz::XYZ;

pub const DEFAULT_GRAVITY: f32 = 9.81;

pub struct KinematicBody {
    pub gravity: f32,
    pub mass: f32,

    pub velocity: XYZ,
}

impl KinematicBody {
    pub fn new(mass: f32) -> KinematicBody {
        KinematicBody {
            gravity: DEFAULT_GRAVITY,
            mass,

            velocity: XYZ::zero(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        // self.velocity.y -= self.gravity * delta;
    }
}
