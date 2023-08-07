use noise::{NoiseFn, OpenSimplex, Perlin, Seedable};

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
        if point[1] < 8 {
            let noise = self.noise_3d.get([
                point[0] as f64 / factor,
                point[1] as f64 / factor,
                point[2] as f64 / factor,
            ]);
            println!("noise: {}", noise);
            if noise > 0.0 {
                if noise < 0.5 {
                    BlockId::Stone
                } else {
                    BlockId::Dirt
                }
            } else {
                BlockId::Air
            }
        } else if point[1] < 10 {
            BlockId::Dirt
        } else if point[1] < 24 {
            let factor = 30.0;
            let height = self
                .noise_2d
                .get([point[0] as f64 / factor, point[2] as f64 / factor])
                .powi(2);

            println!("{} {}", point[1] as f64, ((height + 1.0) / 2.0) * 32.0);
            if (point[1] as f64) < ((height + 1.0) / 2.0) * 24.0 {
                BlockId::Grass
            } else {
                BlockId::Air
            }
        } else {
            BlockId::Air
        }
    }
}
