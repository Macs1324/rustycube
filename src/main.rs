pub mod camera;
pub mod keyboard;
pub mod mesh;
pub mod player;
pub mod transform;
pub mod vertex;
pub mod xyz;

use vertex::Vertex;
use winit::{
    event::Event, event::WindowEvent, event_loop::ControlFlow, event_loop::EventLoop,
    window::WindowBuilder,
};

use glium::Surface;

const VERTEX_SHADER_SRC: &str = r#"
    #version 330
    
    in vec3 position;
    in vec3 normal;
    in vec2 uv;

    uniform mat4 transform;
    uniform mat4 view;
    uniform mat4 projection;

    out vec3 v_position;
    out vec3 v_normal;
    out vec2 v_uv;

    void main() {
        gl_Position = projection * view * transform * vec4(position, 1.0);

        v_position = position;
        v_normal = normal;
        v_uv = uv;
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 330

    in vec3 v_position;
    in vec3 v_normal;
    in vec2 v_uv;

    out vec4 final_color;

    void main() {
        final_color = vec4(v_normal, 1.0);
    }
"#;

fn main() {
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let wb = glium::glutin::window::WindowBuilder::new();
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    let display = glium::Display::new(wb, cb, &event_loop).expect("Failed to create display");
    display.gl_window().window().set_cursor_visible(false);

    let shader_program = glium::program::Program::from_source(
        &display,
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC,
        None,
    )
    .expect("Failed to create shader program");

    let mut keyboard_input = keyboard::Keyboard::new();
    let mut player = player::Player::new(5.0, 10.0);
    player.transform.position.z = 5.0;

    let mut block: mesh::Mesh = mesh::Mesh::empty();
    block.add_quad([
        // BOTTOM
        Vertex::new(0.5, -0.5, 0.5, 0.0, -1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, -1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, -1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, -1.0, 1.0, 0.0, 0.0),
    ]);
    block.add_quad([
        // TOP
        Vertex::new(0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
    ]);
    block.add_quad([
        // FRONT
        Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 0.0),
    ]);
    block.add_quad([
        // BACK
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
    ]);
    block.add_quad([
        // RIGHT
        Vertex::new(0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(0.5, 0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
    ]);
    block.add_quad([
        // LEFT
        Vertex::new(-0.5, 0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.0),
    ]);

    block.build(&display);
    let mut delta: f32 = 1.0 / 120.0;
    player.transform.rotation.y = -90.0f32.to_radians();

    event_loop.run(move |ev, _, control_flow| {
        let frame_start = std::time::Instant::now();
        let next_frame_time = frame_start + std::time::Duration::from_millis(1000 / 120);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        *control_flow = glium::glutin::event_loop::ControlFlow::Poll;

        match &ev {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glium::glutin::event::Event::DeviceEvent { event, .. } => {
                keyboard_input.process_event(&event);
            }
            glium::glutin::event::Event::NewEvents(cause) => match cause {
                glium::glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glium::glutin::event::StartCause::Init => (),
                _ => return,
            },

            glium::glutin::event::Event::MainEventsCleared => {
                let mut target = display.draw();
                target.clear_color(0.1, 0.1, 0.15, 1.0);
                target.clear_depth(1.0);

                block.draw(
                    &mut target,
                    &shader_program,
                    &player.camera,
                    player.transform,
                );

                target.finish().unwrap();
            }
            _ => (),
        }

        player.process_event(&ev, delta);
        player.update(&keyboard_input, delta);
        block.transform.rotation.x += 1.0 * delta;
        block.transform.rotation.y += 1.0 * delta;
        block.transform.rotation.z += 1.0 * delta;
        display
            .gl_window()
            .window()
            .set_cursor_position(glium::glutin::dpi::LogicalPosition::new(400.0, 400.0))
            .unwrap();

        delta = (std::time::Instant::now() - frame_start).as_secs_f32();
    });
}
