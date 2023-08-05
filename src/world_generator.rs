use noise::{NoiseFn, OpenSimplex, Perlin};

use crate::block::BlockId;

pub struct WorldGenerator {
    noise: OpenSimplex,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        let noise = OpenSimplex::new(seed);
        Self { noise }
    }
    pub fn get_block_at(&self, point: [i64; 3]) -> BlockId {
        let noise = self
            .noise
            .get([point[0] as f64, point[1] as f64, point[2] as f64]);
        println!("noise: {}", noise);
        if noise > 0.0 {
            if (noise * 10000.0) % 2.0 == 0.0 {
                BlockId::Grass
            } else {
                BlockId::Grass
            }
        } else {
            BlockId::Air
        }
    }
}
