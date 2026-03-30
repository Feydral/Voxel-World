pub mod block;
pub mod worldgen;
pub mod chunk;

use std::collections::HashMap;

use crate::{math::numerics::int3::Int3, world::{block::Block, chunk::ChunkData, worldgen::TerrainGenerator}};

pub struct WorldData {
    chunks: HashMap<Int3, ChunkData>,
    terrain_generator: TerrainGenerator,
}

impl WorldData {
    pub fn new() -> Self {
        WorldData { 
            chunks: HashMap::new(),
            terrain_generator: TerrainGenerator::new(1),
        }
    }

    pub fn create_chunk(&mut self, position: Int3, overwrite: bool) {
        if overwrite || !self.chunks.contains_key(&position) {
            let chunk = self.terrain_generator.generate_chunk(position);
            self.chunks.insert(position, chunk);
        }
    }

    pub fn set_chunk(&mut self, position: Int3, chunk: ChunkData) {
        self.chunks.insert(position, chunk);
    }

    pub fn get_chunk(&self, position: Int3) -> Option<&ChunkData> {
        self.chunks.get(&position)
    }

    pub fn get_chunk_or_create(&mut self, position: Int3) -> &ChunkData {
        self.create_chunk(position, false);
        self.chunks.get(&position).unwrap()
    }

    pub fn set_block(&mut self, world_pos: Int3, block: Block) {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(local_pos.to_uint3(), block);
        }
    }

    pub fn set_block_or_create(&mut self, world_pos: Int3, block: Block) {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        self.create_chunk(chunk_pos, false);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(local_pos.to_uint3(), block);
        }
    }

    pub fn get_block(&self, world_pos: Int3) -> Option<Block> {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        Some(self.chunks.get(&chunk_pos)?.get_block(local_pos.to_uint3()))
    }

    pub fn get_block_or_create(&mut self, world_pos: Int3) -> Block {
        let chunk_pos = world_pos.div_euclid(ChunkData::SIZE as i32);
        let local_pos = world_pos.rem_euclid(ChunkData::SIZE as i32);
        self.create_chunk(chunk_pos, false);
        self.chunks.get(&chunk_pos).unwrap().get_block(local_pos.to_uint3())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::numerics::uint3::UInt3;

    use super::*;

    #[test]
    fn test_get_block_nonexistent_chunk_returns_none() {
        let world = WorldData::new();
        assert_eq!(world.get_block(Int3::new(0, 0, 0)), None);
    }

    #[test]
    fn test_overwrite_block() {
        let mut world = WorldData::new();
        let pos = Int3::new(5, 0, 5);
        world.set_block_or_create(pos, Block::Debug);
        world.set_block_or_create(pos, Block::Void);
        assert_eq!(world.get_block(pos), Some(Block::Void));
    }

    #[test]
    fn test_adjacent_blocks_dont_alias() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(0, 0, 0), Block::Debug);
        world.set_block_or_create(Int3::new(1, 0, 0), Block::Void);
        assert_eq!(world.get_block(Int3::new(0, 0, 0)), Some(Block::Debug));
    }

    #[test]
    fn test_set_block_calc_origin() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(0, 0, 0), Block::Debug);
        assert_eq!(world.get_chunk(Int3::new(0, 0, 0)).unwrap().get_block(UInt3::new(0, 0, 0)), Block::Debug);
    }

    #[test]
    fn test_set_block_calc_last_in_chunk() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(31, 0, 0), Block::Debug);
        assert_eq!(world.get_chunk(Int3::new(0, 0, 0)).unwrap().get_block(UInt3::new(31, 0, 0)), Block::Debug);
    }

    #[test]
    fn test_set_block_calc_first_in_second_chunk() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(32, 0, 0), Block::Debug);
        assert_eq!(world.get_chunk(Int3::new(1, 0, 0)).unwrap().get_block(UInt3::new(0, 0, 0)), Block::Debug);
    }

    #[test]
    fn test_set_block_calc_first_negative() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(-1, 0, 0), Block::Debug);
        assert_eq!(world.get_chunk(Int3::new(-1, 0, 0)).unwrap().get_block(UInt3::new(31, 0, 0)), Block::Debug);
    }

    #[test]
    fn test_set_block_calc_negative_chunk_boundary() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(-32, 0, 0), Block::Debug);
        assert_eq!(world.get_chunk(Int3::new(-1, 0, 0)).unwrap().get_block(UInt3::new(0, 0, 0)), Block::Debug);
    }

    #[test]
    fn test_get_block_calc_positive() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(33, 0, 0), Block::Debug);
        assert_eq!(world.get_block(Int3::new(33, 0, 0)), Some(Block::Debug));
    }

    #[test]
    fn test_get_block_calc_negative() {
        let mut world = WorldData::new();
        world.set_block_or_create(Int3::new(-33, 0, 0), Block::Debug);
        assert_eq!(world.get_block(Int3::new(-33, 0, 0)), Some(Block::Debug));
    }
}