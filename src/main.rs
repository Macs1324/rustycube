pub mod block;
pub mod camera;
pub mod keyboard;
pub mod mesh;
pub mod player;
pub mod texture_atlas;
pub mod transform;
pub mod vertex;
pub mod xyz;

use std::io::Cursor;

use block::BlockId;
use mesh::Mesh;
use vertex::Vertex;

use glium::{
    glutin::{monitor::VideoMode, window::Fullscreen},
    Surface,
};

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
    out float v_depth;

    void main() {
        gl_Position = projection * view * transform * vec4(position, 1.0);

        v_position = position;
        v_normal = normal;
        v_uv = uv;
        v_depth = gl_Position[3];
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 330

    in vec3 v_position;
    in vec3 v_normal;
    in vec2 v_uv;
    in float v_depth;

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

    let atlas = texture_atlas::TextureAtlas::load(&display, "res/textures/debug.png".to_owned())
        .with_blocks(
            6,
            &vec![
                BlockId::Air,
                BlockId::Dirt,
                BlockId::Grass,
                // BlockId::Stone,
                // BlockId::Sand,
                // BlockId::Water,
            ],
        );

    let mut keyboard_input = keyboard::Keyboard::new();
    let mut player = player::Player::new(5.0, 35.0);
    player.transform.position.z = 5.0;

    let mut blocks: Vec<mesh::Mesh> = Vec::new();
    for i in 0..128 {
        let mut block: mesh::Mesh = mesh::Mesh::empty();
        let nr_blocks = 3.0;
        let block_id = {
            match i % 3 {
                0 => BlockId::Air,
                1 => BlockId::Dirt,
                2 => BlockId::Grass,
                _ => BlockId::Air,
            }
        };
        let w = 1.0 / nr_blocks;
        let h = 1.0 / 6.0;

        let uv = atlas.get_block_uv(block_id);

        block.add_quad([
            // BOTTOM
            Vertex::new(
                0.5,
                -0.5,
                0.5,
                0.0,
                -1.0,
                0.0,
                uv.bottom[0].uv_x,
                uv.bottom[0].uv_y,
            ),
            Vertex::new(
                0.5,
                -0.5,
                -0.5,
                0.0,
                -1.0,
                0.0,
                uv.bottom[1].uv_x,
                uv.bottom[1].uv_y,
            ),
            Vertex::new(
                -0.5,
                -0.5,
                -0.5,
                0.0,
                -1.0,
                0.0,
                uv.bottom[2].uv_x,
                uv.bottom[2].uv_y,
            ),
            Vertex::new(
                -0.5,
                -0.5,
                0.5,
                0.0,
                -1.0,
                0.0,
                uv.bottom[3].uv_x,
                uv.bottom[3].uv_y,
            ),
        ]);
        block.add_quad([
            // TOP
            Vertex::new(
                0.5,
                0.5,
                -0.5,
                0.0,
                1.0,
                1.0,
                uv.top[0].uv_x,
                uv.top[0].uv_y,
            ),
            Vertex::new(0.5, 0.5, 0.5, 0.0, 1.0, 1.0, uv.top[1].uv_x, uv.top[1].uv_y),
            Vertex::new(
                -0.5,
                0.5,
                0.5,
                0.0,
                1.0,
                1.0,
                uv.top[2].uv_x,
                uv.top[2].uv_y,
            ),
            Vertex::new(
                -0.5,
                0.5,
                -0.5,
                0.0,
                1.0,
                1.0,
                uv.top[3].uv_x,
                uv.top[3].uv_y,
            ),
        ]);
        block.add_quad([
            // FRONT
            Vertex::new(
                0.5,
                0.5,
                0.5,
                0.0,
                0.0,
                1.0,
                uv.front[0].uv_x,
                uv.front[0].uv_y,
            ),
            Vertex::new(
                0.5,
                -0.5,
                0.5,
                0.0,
                0.0,
                1.0,
                uv.front[1].uv_x,
                uv.front[1].uv_y,
            ),
            Vertex::new(
                -0.5,
                -0.5,
                0.5,
                0.0,
                0.0,
                1.0,
                uv.front[2].uv_x,
                uv.front[2].uv_y,
            ),
            Vertex::new(
                -0.5,
                0.5,
                0.5,
                0.0,
                0.0,
                1.0,
                uv.front[3].uv_x,
                uv.front[3].uv_y,
            ),
        ]);
        block.add_quad([
            // BACK
            Vertex::new(
                -0.5,
                0.5,
                -0.5,
                0.0,
                0.0,
                -1.0,
                uv.back[0].uv_x,
                uv.back[0].uv_y,
            ),
            Vertex::new(
                -0.5,
                -0.5,
                -0.5,
                0.0,
                0.0,
                -1.0,
                uv.back[1].uv_x,
                uv.back[1].uv_y,
            ),
            Vertex::new(
                0.5,
                -0.5,
                -0.5,
                0.0,
                0.0,
                -1.0,
                uv.back[2].uv_x,
                uv.back[2].uv_y,
            ),
            Vertex::new(
                0.5,
                0.5,
                -0.5,
                0.0,
                0.0,
                -1.0,
                uv.back[3].uv_x,
                uv.back[3].uv_y,
            ),
        ]);
        block.add_quad([
            // RIGHT
            Vertex::new(
                0.5,
                0.5,
                -0.5,
                1.0,
                0.0,
                0.0,
                uv.right[0].uv_x,
                uv.right[0].uv_y,
            ),
            Vertex::new(
                0.5,
                -0.5,
                -0.5,
                1.0,
                0.0,
                0.0,
                uv.right[1].uv_x,
                uv.right[1].uv_y,
            ),
            Vertex::new(
                0.5,
                -0.5,
                0.5,
                1.0,
                0.0,
                0.0,
                uv.right[2].uv_x,
                uv.right[2].uv_y,
            ),
            Vertex::new(
                0.5,
                0.5,
                0.5,
                1.0,
                0.0,
                0.0,
                uv.right[3].uv_x,
                uv.right[3].uv_y,
            ),
        ]);
        block.add_quad([
            // LEFT
            Vertex::new(
                -0.5,
                0.5,
                0.5,
                -1.0,
                0.0,
                0.0,
                uv.left[0].uv_x,
                uv.left[0].uv_y,
            ),
            Vertex::new(
                -0.5,
                -0.5,
                0.5,
                -1.0,
                0.0,
                0.0,
                uv.left[1].uv_x,
                uv.left[1].uv_y,
            ),
            Vertex::new(
                -0.5,
                -0.5,
                -0.5,
                -1.0,
                0.0,
                0.0,
                uv.left[2].uv_x,
                uv.left[2].uv_y,
            ),
            Vertex::new(
                -0.5,
                0.5,
                -0.5,
                -1.0,
                0.0,
                0.0,
                uv.left[3].uv_x,
                uv.left[3].uv_y,
            ),
        ]);
        block.build(&display);
        block.transform.position.x = (i as f32 / 16.0).trunc();
        block.transform.position.z = (i % 16) as f32;
        block.transform.position.y = (i as f32 / 3.0).sin() * 1.0;
        blocks.push(block);
    }

    let mut delta: f32 = 1.0 / 120.0;
    player.transform.rotate_y(-90.0f32.to_radians());

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
                        &atlas.get_texture(),
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
