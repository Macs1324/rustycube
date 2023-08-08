use crate::camera::PerspectiveCamera3D;
use crate::keyboard::Keyboard;
use crate::kinematic_body::KinematicBody;
use crate::transform::Transform;
use crate::util::numbers::move_toward;

pub struct Player {
    pub transform: Transform,
    pub camera: PerspectiveCamera3D,
    pub walk_speed: f32,
    pub lookaround_speed: f32,
    pub kinematic_body: KinematicBody,
    pub walk_acceleration: f32,
    pub walk_deceleration: f32,
}

impl Player {
    pub fn new(
        walk_speed: f32,
        lookaround_speed: f32,
        walk_acceleration: f32,
        walk_deceleration: f32,
    ) -> Player {
        Player {
            transform: Transform::zero(),
            camera: PerspectiveCamera3D::new(16.0 / 9.0, 3.14 / 1.7, 0.01, 1024.0),
            walk_speed,
            lookaround_speed,
            kinematic_body: KinematicBody::new(80.0),
            walk_acceleration,
            walk_deceleration,
        }
    }

    pub fn update(&mut self, input: &Keyboard, delta_time: f32) {
        use glium::glutin::event::VirtualKeyCode;
        self.kinematic_body.update(delta_time);

        if input.is_key_pressed(VirtualKeyCode::W) {
            self.kinematic_body.velocity.z = move_toward(
                self.kinematic_body.velocity.z,
                -self.walk_speed,
                self.walk_acceleration * delta_time,
            );
        } else {
            self.kinematic_body.velocity.z = move_toward(
                self.kinematic_body.velocity.z,
                0.0,
                self.walk_deceleration * delta_time,
            );
        }

        if input.is_key_pressed(VirtualKeyCode::S) {
            self.kinematic_body.velocity.z = move_toward(
                self.kinematic_body.velocity.z,
                self.walk_speed,
                self.walk_acceleration * delta_time,
            );
        } else {
            self.kinematic_body.velocity.z = move_toward(
                self.kinematic_body.velocity.z,
                0.0,
                self.walk_deceleration * delta_time,
            );
        }

        if input.is_key_pressed(VirtualKeyCode::A) {
            self.kinematic_body.velocity.x = move_toward(
                self.kinematic_body.velocity.x,
                -self.walk_speed,
                self.walk_acceleration * delta_time,
            );
        } else {
            self.kinematic_body.velocity.x = move_toward(
                self.kinematic_body.velocity.x,
                0.0,
                self.walk_deceleration * delta_time,
            );
        }

        if input.is_key_pressed(VirtualKeyCode::D) {
            self.kinematic_body.velocity.x = move_toward(
                self.kinematic_body.velocity.x,
                self.walk_speed,
                self.walk_acceleration * delta_time,
            );
        } else {
            self.kinematic_body.velocity.x = move_toward(
                self.kinematic_body.velocity.x,
                0.0,
                self.walk_deceleration * delta_time,
            );
        }

        if input.is_key_pressed(VirtualKeyCode::Space) {
            self.kinematic_body.velocity.y = move_toward(
                self.kinematic_body.velocity.y,
                self.walk_speed,
                self.walk_acceleration * delta_time,
            );
        } else {
            self.kinematic_body.velocity.y = move_toward(
                self.kinematic_body.velocity.y,
                0.0,
                self.walk_deceleration * delta_time,
            );
        }
        if input.is_key_pressed(VirtualKeyCode::C) {
            self.kinematic_body.velocity.y = move_toward(
                self.kinematic_body.velocity.y,
                -self.walk_speed,
                self.walk_acceleration * delta_time,
            );
        } else {
            self.kinematic_body.velocity.y = move_toward(
                self.kinematic_body.velocity.y,
                0.0,
                self.walk_deceleration * delta_time,
            );
        }

        println!("velocity: {:?}", self.kinematic_body.velocity);
        self.transform.position = self.transform.position
            + self
                .kinematic_body
                .velocity
                .rotated_y(self.transform.get_rotation().y + 90.0f32.to_radians())
                * delta_time;
    }

    pub fn process_event(&mut self, ev: &glium::glutin::event::Event<()>, delta_time: f32) {
        match ev {
            glium::glutin::event::Event::DeviceEvent { event, .. } => match event {
                glium::glutin::event::DeviceEvent::MouseMotion { delta } => {
                    let d_x = delta.0 as f32;
                    let d_y = delta.1 as f32;

                    self.transform
                        .rotate_y(d_x * self.lookaround_speed * delta_time);
                    self.transform
                        .rotate_x(-d_y * self.lookaround_speed * delta_time);
                }

                _ => (),
            },
            _ => (),
        }
    }
    fn forward(&mut self, speed: f32) {
        self.transform.position.z += speed * self.transform.get_rotation().y.sin();
        self.transform.position.x += speed * self.transform.get_rotation().y.cos();
    }

    fn backward(&mut self, speed: f32) {
        self.transform.position.z -= speed * self.transform.get_rotation().y.sin();
        self.transform.position.x -= speed * self.transform.get_rotation().y.cos();
    }

    fn left(&mut self, speed: f32) {
        self.transform.position.x += speed * self.transform.get_rotation().y.sin();
        self.transform.position.z -= speed * self.transform.get_rotation().y.cos();
    }

    fn right(&mut self, speed: f32) {
        self.transform.position.x -= speed * self.transform.get_rotation().y.sin();
        self.transform.position.z += speed * self.transform.get_rotation().y.cos();
    }
}
