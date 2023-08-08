use noise::{NoiseFn, OpenSimplex, Perlin, Seedable};

const ROCK_MAX_HEIGHT: i64 = 100;
const DIRT_LAYER_HEIGHT: i64 = 120;
const HILL_LAYER_HEIGHT: i64 = 148;
const HILL_MAX_HEIGHT: i64 = 32;

const TOTAL_HEIGHT: i64 = ROCK_MAX_HEIGHT + DIRT_LAYER_HEIGHT + HILL_LAYER_HEIGHT + HILL_MAX_HEIGHT;

use crate::block::BlockId;

pub struct WorldGenerator {
    noise_3d: Box<dyn NoiseFn<f64, 3>>,
    noise_2d: Box<dyn NoiseFn<f64, 2>>,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        let noise_3d = Box::new(noise::Perlin::new(seed));
        let noise_2d = Box::new(noise::Perlin::new(seed));
        Self { noise_3d, noise_2d }
    }
    pub fn get_block_at(&self, point: [i64; 3]) -> BlockId {
        let factor = 5.0;
        if point[1] < ROCK_MAX_HEIGHT {
            let noise = self.noise_3d.get([
                point[0] as f64 / factor,
                point[1] as f64 / factor,
                point[2] as f64 / factor,
            ]);
            if noise > 0.0 {
                if noise < 0.5 {
                    BlockId::Stone
                } else {
                    BlockId::Dirt
                }
            } else {
                BlockId::Air
            }
        } else if point[1] < DIRT_LAYER_HEIGHT {
            BlockId::Dirt
        } else if point[1] < HILL_LAYER_HEIGHT {
            let factor = 30.0;
            let height = self
                .noise_2d
                .get([point[0] as f64 / factor, point[2] as f64 / factor]);

            if (point[1] as f64)
                < DIRT_LAYER_HEIGHT as f64 + ((height + 1.0) / 2.0) * HILL_MAX_HEIGHT as f64
            {
                BlockId::Grass
            } else {
                BlockId::Air
            }
        } else {
            BlockId::Air
        }
    }
}
