use std::println;

use crate::{
    block::BlockId,
    mesh::Mesh,
    texture_atlas::{BlockUv, TextureAtlas},
    transform::Transform,
    vertex::Vertex,
};

const CHUNK_HEIGHT: usize = 32;
const CHUNK_WIDTH: usize = 32;
const CHUNK_DEPTH: usize = 32;

pub enum ChunkNeighbor {
    LeftNeighbor,
    RightNeighbor,
    FrontNeighbor,
    BackNeighbor,
}

pub struct Chunk {
    pub transform: Transform,
    pub data: [[[BlockId; CHUNK_DEPTH]; CHUNK_WIDTH]; CHUNK_HEIGHT],
}

impl Chunk {
    pub fn new(transform: &Transform) -> Chunk {
        Chunk {
            transform: *transform,
            data: [[[BlockId::Air; CHUNK_DEPTH]; CHUNK_WIDTH]; CHUNK_HEIGHT],
        }
    }

    pub fn generate_data(&mut self) {
        for h in 0..CHUNK_HEIGHT {
            for w in 0..CHUNK_WIDTH {
                for d in 0..CHUNK_DEPTH {
                    self.data[h][w][d] = BlockId::Air;
                    if h > 3 && h < 8 {
                        self.data[h][w][d] = BlockId::Grass;
                    }
                }
            }
        }
    }

    pub fn generate_mesh(
        &self,
        neighbor_left: Option<&Chunk>,
        neighbor_right: Option<&Chunk>,
        neighbor_front: Option<&Chunk>,
        neighbor_back: Option<&Chunk>,
        atlas: &TextureAtlas,
    ) -> Mesh {
        let mut mesh = Mesh::empty();

        for h in 0..CHUNK_HEIGHT {
            for w in 0..CHUNK_WIDTH {
                for d in 0..CHUNK_DEPTH {
                    let block_id: BlockId = self.data[h][w][d];
                    println!("Adding block: {:?} at ({}, {}, {})", block_id, w, h, d);
                    if block_id == BlockId::Air {
                        continue;
                    }
                    let uv = atlas.get_block_uv(block_id);
                    // Top
                    if let Some(upper_layer) = self.data.get(h + 1) {
                        if upper_layer[w][d] == BlockId::Air {
                            self.add_upper_quad_for_block(&mut mesh, h, w, d, uv);
                        }
                    } else {
                        self.add_upper_quad_for_block(&mut mesh, h, w, d, uv);
                    }

                    if let Some(front_block) = self.data[h][w].get(d + 1) {
                        if *front_block == BlockId::Air {
                            self.add_front_quad_for_block(&mut mesh, h, w, d, uv);
                        }
                    } else {
                        self.add_front_quad_for_block(&mut mesh, h, w, d, uv);
                    }

                    if let Some(right_row) = self.data[h].get(w + 1) {
                        if right_row[d] == BlockId::Air {
                            self.add_right_quad_for_block(&mut mesh, h, w, d, uv);
                        }
                    } else {
                        self.add_right_quad_for_block(&mut mesh, h, w, d, uv);
                    }
                }
            }
        }

        mesh
    }

    fn add_upper_quad_for_block(
        &self,
        mesh: &mut Mesh,
        h: usize,
        w: usize,
        d: usize,
        uv: &BlockUv,
    ) {
        // TOP
        mesh.add_quad([
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 - 0.5,
                0.0,
                1.0,
                0.0,
                uv.top[0].uv_x,
                uv.top[0].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.top[1].uv_x,
                uv.top[1].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 - 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.top[2].uv_x,
                uv.top[2].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 - 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 - 0.5,
                0.0,
                1.0,
                0.0,
                uv.top[3].uv_x,
                uv.top[3].uv_y,
            ),
        ]);
    }
    fn add_right_quad_for_block(
        &self,
        mesh: &mut Mesh,
        h: usize,
        w: usize,
        d: usize,
        uv: &BlockUv,
    ) {
        // TOP
        mesh.add_quad([
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 - 0.5,
                0.0,
                1.0,
                0.0,
                uv.right[0].uv_x,
                uv.right[0].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 - 0.5,
                self.transform.position.z + d as f32 - 0.5,
                0.0,
                1.0,
                0.0,
                uv.right[1].uv_x,
                uv.right[1].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 - 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.right[2].uv_x,
                uv.right[2].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.right[3].uv_x,
                uv.right[3].uv_y,
            ),
        ]);
    }
    fn add_front_quad_for_block(
        &self,
        mesh: &mut Mesh,
        h: usize,
        w: usize,
        d: usize,
        uv: &BlockUv,
    ) {
        // TOP
        mesh.add_quad([
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.front[0].uv_x,
                uv.front[0].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 + 0.5,
                self.transform.position.y + h as f32 - 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.front[1].uv_x,
                uv.front[1].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 - 0.5,
                self.transform.position.y + h as f32 - 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.front[2].uv_x,
                uv.front[2].uv_y,
            ),
            Vertex::new(
                self.transform.position.x + w as f32 - 0.5,
                self.transform.position.y + h as f32 + 0.5,
                self.transform.position.z + d as f32 + 0.5,
                0.0,
                1.0,
                0.0,
                uv.front[3].uv_x,
                uv.front[3].uv_y,
            ),
        ]);
    }
}
