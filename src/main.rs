pub mod block;
pub mod camera;
pub mod chunk;
pub mod keyboard;
pub mod mesh;
pub mod player;
pub mod texture_atlas;
pub mod transform;
pub mod vertex;
pub mod world;
pub mod world_generator;
pub mod xyz;

use std::{
    io::Cursor,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use block::BlockId;
use chunk::Chunk;
use mesh::Mesh;
use transform::Transform;
use vertex::Vertex;

use glium::{
    glutin::{monitor::VideoMode, platform::unix::x11::ffi::CurrentTime, window::Fullscreen},
    Surface,
};
use world_generator::WorldGenerator;

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
        v_normal = transpose(inverse(mat3(transform))) * normal;
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



    vec4 color_light = vec4(1.0, 1.0, 1.0, 1.0);
    vec4 color_dark = vec4(0.4, 0.4, 0.45, 1.0);
    vec3 direction_light = vec3(1.0, 10.0, 1.0);

    void main() {
        final_color = texture(albedo, v_uv);
        float light_amount = dot(normalize(v_normal), normalize(direction_light));
        light_amount = clamp(light_amount, 0.0, 1.0);

        final_color = mix(final_color * color_dark, final_color, light_amount);
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

    let atlas = texture_atlas::TextureAtlas::load(&display, "res/textures/blocks.png".to_owned())
        .with_blocks(
            6,
            &vec![
                // BlockId::Air,
                BlockId::Dirt,
                BlockId::Grass,
                BlockId::Stone,
                // BlockId::Sand,
                // BlockId::Water,
            ],
        );

    let mut keyboard_input = keyboard::Keyboard::new();
    let mut player = player::Player::new(8.0, 180.0);
    player.transform.position.z = 5.0;

    let mut chunk = Chunk::new(&Transform::zero());
    let generator = WorldGenerator::new({
        let start = SystemTime::now();
        start
            .duration_since(UNIX_EPOCH)
            .expect("King crimson is among us")
            .as_millis() as u32
    });
    chunk.generate_data(&generator);
    let mut chunk_mesh = chunk.generate_mesh(None, None, None, None, &atlas);
    chunk_mesh.build(&display);

    let mut chunks: Vec<Chunk> = Vec::new();
    let mut chunk_meshes: Vec<Mesh> = Vec::new();
    let render_distance = 10;

    for i in 0..render_distance {
        for j in 0..render_distance {
            let mut transform = Transform::zero();
            transform.position.x = i as f32;
            transform.position.z = j as f32;
            let mut chunk = Chunk::new(&transform);

            chunk.generate_data(&generator);
            let mut chunk_mesh = chunk.generate_mesh(None, None, None, None, &atlas);
            chunk_mesh.build(&display);

            chunk_meshes.push(chunk_mesh);
            chunks.push(chunk);
        }
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

                chunk_mesh.draw(
                    &mut target,
                    &shader_program,
                    &atlas.get_texture(),
                    &player.camera,
                    player.transform,
                );

                for mesh in &chunk_meshes {
                    mesh.draw(
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
