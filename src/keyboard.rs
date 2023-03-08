use std::collections::HashMap;

use glium::glutin::event::VirtualKeyCode;

struct KeyState {
    just_pressed: bool,
    pressed: bool,
    just_released: bool,
    released: bool,
}

pub struct Keyboard {
    states: HashMap<VirtualKeyCode, KeyState>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            states: HashMap::new(),
        }
    }

    pub fn process_event(&mut self, ev: &glium::glutin::event::DeviceEvent) {
        match ev {
            glium::glutin::event::DeviceEvent::Key(key) => {
                if key.virtual_keycode.is_some() {
                    let keycode = key.virtual_keycode.unwrap();
                    match key.state {
                        glium::glutin::event::ElementState::Pressed => {
                            if self.states.contains_key(&keycode)
                                && self.states[&keycode].just_pressed
                            {
                                self.states
                                    .entry(keycode)
                                    .and_modify(|x| {
                                        x.just_pressed = false;
                                        x.pressed = true;
                                        x.released = false;
                                        x.just_released = false;
                                    })
                                    .or_insert(KeyState {
                                        just_pressed: false,
                                        pressed: true,
                                        just_released: false,
                                        released: false,
                                    });
                            } else {
                                self.states
                                    .entry(keycode)
                                    .and_modify(|x| {
                                        x.just_pressed = true;
                                        x.pressed = true;
                                        x.released = false;
                                        x.just_released = false;
                                    })
                                    .or_insert(KeyState {
                                        just_pressed: true,
                                        pressed: true,
                                        just_released: false,
                                        released: false,
                                    });
                            }
                        }
                        glium::glutin::event::ElementState::Released => {
                            if self.states[&keycode].just_released {
                                self.states
                                    .entry(keycode)
                                    .and_modify(|x| {
                                        x.just_pressed = false;
                                        x.pressed = false;
                                        x.released = true;
                                        x.just_released = true;
                                    })
                                    .or_insert(KeyState {
                                        just_pressed: false,
                                        pressed: false,
                                        just_released: true,
                                        released: true,
                                    });
                            } else {
                                self.states
                                    .entry(keycode)
                                    .and_modify(|x| {
                                        x.just_pressed = false;
                                        x.pressed = false;
                                        x.released = true;
                                        x.just_released = false;
                                    })
                                    .or_insert(KeyState {
                                        just_pressed: false,
                                        pressed: false,
                                        just_released: false,
                                        released: true,
                                    });
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    pub fn is_key_pressed(&self, keycode: glium::glutin::event::VirtualKeyCode) -> bool {
        self.states.contains_key(&keycode) && self.states[&keycode].pressed
    }
    pub fn is_key_released(&self, keycode: glium::glutin::event::VirtualKeyCode) -> bool {
        self.states.contains_key(&keycode) && self.states[&keycode].released
    }
    pub fn is_key_just_pressed(&self, keycode: glium::glutin::event::VirtualKeyCode) -> bool {
        self.states.contains_key(&keycode) && self.states[&keycode].just_pressed
    }
    pub fn is_key_just_released(&self, keycode: glium::glutin::event::VirtualKeyCode) -> bool {
        self.states.contains_key(&keycode) && self.states[&keycode].just_released
    }
}
