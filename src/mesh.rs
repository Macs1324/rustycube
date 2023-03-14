use crate::camera::Camera;
use crate::transform;
use crate::transform::Transform;
use crate::vertex::Vertex;
use glium::{
    backend::Facade,
    program::Program,
    uniform,
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    IndexBuffer, Surface, VertexBuffer,
};

pub struct Mesh {
    pub transform: Transform,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    vbo: Option<VertexBuffer<Vertex>>,
    vao: Option<IndexBuffer<u32>>,
}

impl Mesh {
    pub fn empty() -> Mesh {
        Mesh {
            transform: Transform::zero(),
            vertices: Vec::new(),
            indices: Vec::new(),

            vbo: None,
            vao: None,
        }
    }

    pub fn build(&mut self, facade: &dyn Facade) {
        println!("{:?}", self.indices);
        println!("{:?}", self.vertices);
        self.vbo = Some(VertexBuffer::new(facade, &self.vertices).expect("Failed to create vbo"));
        self.vao = Some(
            IndexBuffer::new(
                facade,
                glium::index::PrimitiveType::TrianglesList,
                &self.indices,
            )
            .expect("Failed to create vao"),
        );
    }

    pub fn draw<S: Surface>(
        &self,
        surface: &mut S,
        shader_program: &Program,
        texture: &glium::texture::SrgbTexture2d,
        camera: &dyn Camera,
        camera_transform: Transform,
    ) {
        if self.vbo.is_none() && self.vao.is_none() {
            return;
        }
        surface
            .draw(
                self.vbo.as_ref().unwrap(),
                self.vao.as_ref().unwrap(),
                shader_program,
                &uniform! {
                    transform : transform::mat2array(self.transform.to_matrix()),
                    view : transform::mat2array(camera.view(camera_transform)),
                    projection : transform::mat2array(camera.projection()),
                    albedo : glium::uniforms::Sampler(texture, glium::uniforms::SamplerBehavior {
                        minify_filter : MinifySamplerFilter::Nearest,
                        magnify_filter : MagnifySamplerFilter::Nearest,
                        ..Default::default()
                    })
                },
                &glium::draw_parameters::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .expect("Failed to draw mesh");
    }

    pub fn add_quad(&mut self, quad: [Vertex; 4]) {
        for vertex in quad {
            self.vertices.push(vertex);
        }

        let mut len = self
            .indices
            .iter()
            .copied()
            .reduce(|max, i| if i > max { i } else { max })
            .unwrap_or(0u32);
        if len != 0 {
            len += 1u32;
        }

        self.indices.push(len + 0);
        self.indices.push(len + 1);
        self.indices.push(len + 2);

        self.indices.push(len + 0);
        self.indices.push(len + 2);
        self.indices.push(len + 3);
    }
}
