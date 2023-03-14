use crate::camera::PerspectiveCamera3D;
use crate::keyboard::Keyboard;
use crate::transform::Transform;

pub struct Player {
    pub transform: Transform,
    pub camera: PerspectiveCamera3D,
    pub walk_speed: f32,
    pub lookaround_speed: f32,
}

impl Player {
    pub fn new(walk_speed: f32, lookaround_speed: f32) -> Player {
        Player {
            transform: Transform::zero(),
            camera: PerspectiveCamera3D::new(1.0, 3.14 / 3.0, 0.01, 1024.0),
            walk_speed,
            lookaround_speed,
        }
    }

    pub fn update(&mut self, input: &Keyboard, delta_time: f32) {
        use glium::glutin::event::VirtualKeyCode;

        if input.is_key_pressed(VirtualKeyCode::W) {
            self.forward(self.walk_speed * delta_time);
        }

        if input.is_key_pressed(VirtualKeyCode::S) {
            self.backward(self.walk_speed * delta_time);
        }

        if input.is_key_pressed(VirtualKeyCode::A) {
            self.left(self.walk_speed * delta_time);
        }

        if input.is_key_pressed(VirtualKeyCode::D) {
            self.right(self.walk_speed * delta_time);
        }

        if input.is_key_pressed(VirtualKeyCode::Space) {
            self.transform.position.y += self.walk_speed * delta_time;
        }
        if input.is_key_pressed(VirtualKeyCode::C) {
            self.transform.position.y -= self.walk_speed * delta_time;
        }
    }

    pub fn process_event(&mut self, ev: &glium::glutin::event::Event<()>, delta_time: f32) {
        match ev {
            glium::glutin::event::Event::DeviceEvent { event, .. } => match event {
                glium::glutin::event::DeviceEvent::MouseMotion { delta } => {
                    let d_x = delta.0 as f32;
                    let d_y = delta.1 as f32;

                    self.transform.rotation.y += d_x * self.lookaround_speed * delta_time;
                    self.transform.rotation.x -= d_y * self.lookaround_speed * delta_time;
                }

                _ => (),
            },
            _ => (),
        }
    }
    fn forward(&mut self, speed: f32) {
        self.transform.position.z += speed * self.transform.rotation.y.sin();
        self.transform.position.x += speed * self.transform.rotation.y.cos();
    }

    fn backward(&mut self, speed: f32) {
        self.transform.position.z -= speed * self.transform.rotation.y.sin();
        self.transform.position.x -= speed * self.transform.rotation.y.cos();
    }

    fn left(&mut self, speed: f32) {
        self.transform.position.x += speed * self.transform.rotation.y.sin();
        self.transform.position.z -= speed * self.transform.rotation.y.cos();
    }

    fn right(&mut self, speed: f32) {
        self.transform.position.x -= speed * self.transform.rotation.y.sin();
        self.transform.position.z += speed * self.transform.rotation.y.cos();
    }
}
