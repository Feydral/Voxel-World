use std::collections::HashMap;

use crate::{math::numerics::int3::Int3, world::{block::Block, chunk::ChunkData}};

pub mod block;
pub mod worldgen;
pub mod chunk;

pub struct WorldData {
    chunks: HashMap<Int3, ChunkData>,
}

impl WorldData {
    pub fn new() -> Self {
        WorldData { 
            chunks: HashMap::new(),
        }
    }

    pub fn generate_chunk(&mut self, position: Int3, overwrite: bool) {
        if overwrite || !self.chunks.contains_key(&position) {
            let chunk = worldgen::generate_chunk(position);
            self.chunks.insert(position, chunk);
        }
    }

    pub fn set_chunk(&mut self, position: Int3, chunk: ChunkData) {
        self.chunks.insert(position, chunk);
    }

    pub fn get_chunk(&self, position: Int3) -> Option<&ChunkData> {
        self.chunks.get(&position)
    }

    pub fn get_chunk_or_generate(&mut self, position: Int3) -> &ChunkData {
        self.generate_chunk(position, false);
        self.chunks.get(&position).unwrap()
    }

    pub fn set_block(&mut self, world_pos: Int3, block: Block) {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(local_pos, block);
        }
    }

    pub fn set_block_or_generate(&mut self, world_pos: Int3, block: Block) {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        self.generate_chunk(chunk_pos, false);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(local_pos, block);
        }
    }

    pub fn get_block(&self, world_pos: Int3) -> Option<Block> {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        Some(self.chunks.get(&chunk_pos)?.get_block(local_pos))
    }

    pub fn get_block_or_generate(&mut self, world_pos: Int3) -> Block {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        self.generate_chunk(chunk_pos, false);
        self.chunks.get(&chunk_pos).unwrap().get_block(local_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_block_positive() {
        let mut world = WorldData::new();
        let pos = Int3::new(1, 0, 1);
        world.set_block_or_generate(pos, Block::Debug);
        assert_eq!(world.get_block(pos), Some(Block::Debug));
    }

    #[test]
    fn test_set_get_block_negative() {
        let mut world = WorldData::new();
        let pos = Int3::new(-1, 0, -1);
        world.set_block_or_generate(pos, Block::Debug);
        assert_eq!(world.get_block(pos), Some(Block::Debug));
    }

    #[test]
    fn test_chunk_boundary() {
        let mut world = WorldData::new();
        let a = Int3::new(31, 0, 0);
        let b = Int3::new(32, 0, 0);
        world.set_block_or_generate(a, Block::Debug);
        world.set_block_or_generate(b, Block::Debug);
        assert_eq!(world.get_block(a), Some(Block::Debug));
        assert_eq!(world.get_block(b), Some(Block::Debug));
    }

    #[test]
    fn test_negative_chunk_boundary() {
        let mut world = WorldData::new();
        let a = Int3::new(-1, 0, 0);
        let b = Int3::new(-32, 0, 0);
        let c = Int3::new(-33, 0, 0);
        world.set_block_or_generate(a, Block::Debug);
        world.set_block_or_generate(b, Block::Debug);
        world.set_block_or_generate(c, Block::Debug);
        assert_eq!(world.get_block(a), Some(Block::Debug));
        assert_eq!(world.get_block(b), Some(Block::Debug));
        assert_eq!(world.get_block(c), Some(Block::Debug));
    }

    #[test]
    fn test_blocks_dont_alias() {
        let mut world = WorldData::new();
        let a = Int3::new(0, 0, 0);
        let b = Int3::new(1, 0, 0);
        world.set_block_or_generate(a, Block::Debug);
        world.set_block_or_generate(b, Block::Void);
        assert_eq!(world.get_block(a), Some(Block::Debug));
    }
}