use crate::xyz::XYZ;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        normal_x: f32,
        normal_y: f32,
        normal_z: f32,
        uv_x: f32,
        uv_y: f32,
    ) -> Vertex {
        Vertex {
            position: [pos_x, pos_y, pos_z],
            normal: [normal_x, normal_y, normal_z],
            uv: [uv_x, uv_y],
        }
    }

    pub fn new_xyz(pos: XYZ, norm: XYZ, uv: [f32; 2]) -> Vertex {
        Vertex {
            position: [pos.x, pos.y, pos.z],
            normal: [norm.x, norm.y, norm.z],
            uv,
        }
    }
}
glium::implement_vertex!(Vertex, position, normal, uv);
