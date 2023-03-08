use glium::{glutin::window::Fullscreen, Surface};

pub mod camera;
pub mod keyboard;
pub mod mesh;
pub mod player;
pub mod transform;
pub mod vertex;
pub mod xyz;

use vertex::Vertex;

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
    let mut player = player::Player::new(0.01, 0.005);
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

    event_loop.run(move |ev, _, control_flow| {
        player.process_event(&ev);
        player.update(&keyboard_input);

        match ev {
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
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color(0.1, 0.1, 0.15, 1.0);
        target.clear_depth(1.0);

        block.draw(
            &mut target,
            &shader_program,
            &player.camera,
            player.transform,
        );

        block.transform.rotation.x += 0.001;
        block.transform.rotation.y += 0.001;
        block.transform.rotation.z += 0.001;

        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        display
            .gl_window()
            .window()
            .set_cursor_position(glium::glutin::dpi::LogicalPosition::new(400.0, 400.0))
            .unwrap();
    });
}
