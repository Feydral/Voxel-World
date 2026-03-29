use opensimplex_noise_rs::OpenSimplexNoise;
use crate::{math::numerics::int3::Int3, world::{block::Block, chunk::ChunkData}};

pub struct TerrainGenerator {
    noise: OpenSimplexNoise,
    scale: f64,
    amplitude: f64,
    cave_scale: f64,
    cave_threshold: f64,
}

impl TerrainGenerator {
    pub fn new(seed: i64) -> Self {
        Self {
            noise: OpenSimplexNoise::new(Some(seed)),
            scale: 0.01,
            amplitude: 32.0,
            cave_scale: 0.05,
            cave_threshold: 0.4,
        }
    }

    fn surface_height(&self, world_x: i32, world_z: i32) -> i32 {
        let n = self.noise.eval_2d(
            world_x as f64 * self.scale,
            world_z as f64 * self.scale,
        );
        ((n + 1.0) * 0.5 * self.amplitude) as i32
    }

    pub fn generate_chunk(&self, position: Int3) -> ChunkData {
        let mut chunk = ChunkData::new_empty();

        for x in 0..ChunkData::SIZE {
            for z in 0..ChunkData::SIZE {
                let world_x = position.x * ChunkData::SIZE as i32 + x as i32;
                let world_z = position.z * ChunkData::SIZE as i32 + z as i32;
                let surface = self.surface_height(world_x, world_z);

                for y in 0..ChunkData::SIZE {
                    let world_y = position.y * ChunkData::SIZE as i32 + y as i32;

                    let block = if world_y > surface {
                        Block::Air
                    } else if world_y == surface {
                        Block::GrassBlock
                    } else if world_y >= surface - 3 {
                        Block::DirtBlock
                    } else {
                        Block::StoneBlock
                    };

                    chunk.set_block(Int3::new(x as i32, y as i32, z as i32), block);
                }
            }
        }

        chunk
    }
}