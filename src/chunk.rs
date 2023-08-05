use crate::{block::BlockId, transform::Transform, vertex::Vertex};

const CHUNK_HEIGHT: usize = 128;
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

    pub fn generateVertexData(
        &self,
        neighbor_left: &Chunk,
        neighbor_right: &Chunk,
        neighbor_front: &Chunk,
        neighbor_back: &Chunk,
    ) -> Vec<Vertex> {
        Vec::new()
    }
}
