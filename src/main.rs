pub mod camera;
pub mod keyboard;
pub mod mesh;
pub mod player;
pub mod texture_atlas;
pub mod transform;
pub mod vertex;
pub mod xyz;

use std::io::Cursor;

use mesh::Mesh;
use vertex::Vertex;

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

    uniform sampler2D albedo;

    out vec4 final_color;

    void main() {
        final_color = texture(albedo, v_uv);
    }
"#;

fn main() {
    let cb = glium::glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
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

    let mut blocks: Vec<mesh::Mesh> = Vec::new();
    for i in 0..128 {
        let mut block: mesh::Mesh = mesh::Mesh::empty();
        let nr_blocks = 3.0;
        let block_id = i as f32 % nr_blocks;
        let w = 1.0 / nr_blocks;
        let h = 1.0 / 6.0;

        block.add_quad([
            // BOTTOM
            Vertex::new(0.5, -0.5, 0.5, 0.0, -1.0, 0.0, w * block_id + w, h * 5.0),
            Vertex::new(0.5, -0.5, -0.5, 0.0, -1.0, 0.0, w * block_id + w, h * 4.0),
            Vertex::new(-0.5, -0.5, -0.5, 0.0, -1.0, 0.0, w * block_id, h * 4.0),
            Vertex::new(-0.5, -0.5, 0.5, 0.0, -1.0, 0.0, w * block_id, h * 5.0),
        ]);
        block.add_quad([
            // TOP
            Vertex::new(0.5, 0.5, -0.5, 0.0, 1.0, 1.0, w * block_id + w, 0.0),
            Vertex::new(0.5, 0.5, 0.5, 0.0, 1.0, 1.0, w * block_id + w, h),
            Vertex::new(-0.5, 0.5, 0.5, 0.0, 1.0, 1.0, w * block_id, h),
            Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0, 1.0, w * block_id, 0.0),
        ]);
        block.add_quad([
            // FRONT
            Vertex::new(0.5, 0.5, 0.5, 1.0, 1.0, 1.0, w * block_id + w, h * 2.0),
            Vertex::new(0.5, -0.5, 0.5, 1.0, 1.0, 1.0, w * block_id + w, h * 3.0),
            Vertex::new(-0.5, -0.5, 0.5, 1.0, 1.0, 1.0, w * block_id, h * 3.0),
            Vertex::new(-0.5, 0.5, 0.5, 1.0, 1.0, 1.0, w * block_id, h * 2.0),
        ]);
        block.add_quad([
            // BACK
            Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0, 1.0, w * block_id + w, h * 5.0),
            Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0, 1.0, w * block_id + w, h * 6.0),
            Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0, 1.0, w * block_id, h * 6.0),
            Vertex::new(0.5, 0.5, -0.5, 0.0, 1.0, 1.0, w * block_id, h * 5.0),
        ]);
        block.add_quad([
            // RIGHT
            Vertex::new(0.5, 0.5, -0.5, 0.0, 1.0, 1.0, w * block_id + w, h * 3.0),
            Vertex::new(0.5, -0.5, -0.5, 0.0, 1.0, 1.0, w * block_id + w, h * 4.0),
            Vertex::new(0.5, -0.5, 0.5, 0.0, 1.0, 1.0, w * block_id, h * 4.0),
            Vertex::new(0.5, 0.5, 0.5, 0.0, 1.0, 1.0, w * block_id, h * 3.0),
        ]);
        block.add_quad([
            // LEFT
            Vertex::new(-0.5, 0.5, 0.5, 0.0, 1.0, 1.0, w * block_id + w, h * 1.0),
            Vertex::new(-0.5, -0.5, 0.5, 0.0, 1.0, 1.0, w * block_id + w, h * 2.0),
            Vertex::new(-0.5, -0.5, -0.5, 0.0, 1.0, 1.0, w * block_id, h * 2.0),
            Vertex::new(-0.5, 0.5, -0.5, 0.0, 1.0, 1.0, w * block_id, h * 1.0),
        ]);
        block.build(&display);
        block.transform.position.x = (i as f32 / 16.0).trunc();
        block.transform.position.z = (i % 16) as f32;
        block.transform.position.y = (i as f32).sin() * 1.0;
        blocks.push(block);
    }

    let image = image::load(
        Cursor::new(&include_bytes!("res/textures/debug.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut delta: f32 = 1.0 / 120.0;
    player.transform.rotation.y = -90.0f32.to_radians();

    event_loop.run(move |ev, _, control_flow| {
        let frame_start = std::time::Instant::now();
        let next_frame_time = frame_start + std::time::Duration::from_millis(1000 / 120);
        // *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

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
                target.clear_color(0.8, 0.85, 1.0, 1.0);
                target.clear_depth(1.0);

                for block in &blocks {
                    block.draw(
                        &mut target,
                        &shader_program,
                        &texture,
                        &player.camera,
                        player.transform,
                    );
                }

                target.finish().unwrap();
            }
            _ => (),
        }

        player.process_event(&ev, delta);
        player.update(&keyboard_input, delta);
        // block.transform.rotation.x += 1.0 * delta;
        // block.transform.rotation.y += 1.0 * delta;
        // block.transform.rotation.z += 1.0 * delta;
        display
            .gl_window()
            .window()
            .set_cursor_position(glium::glutin::dpi::LogicalPosition::new(400.0, 400.0))
            .unwrap();

        delta = (std::time::Instant::now() - frame_start).as_secs_f32();
    });
}
