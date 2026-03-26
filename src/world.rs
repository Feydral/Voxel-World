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
            let chunk = worldgen::generate_chunk();
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
        let chunk_pos = world_pos / ChunkData::SIZE as i32;
        let local_pos = world_pos % ChunkData::SIZE as i32;
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(local_pos, block);
        }
    }

    pub fn set_block_or_generate(&mut self, world_pos: Int3, block: Block) {
        let chunk_pos = world_pos / ChunkData::SIZE as i32;
        let local_pos = world_pos % ChunkData::SIZE as i32;
        self.generate_chunk(chunk_pos, false);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.set_block(local_pos, block);
        }
    }

    pub fn get_block(&self, world_pos: Int3) -> Option<Block> {
        let chunk_pos = world_pos / ChunkData::SIZE as i32;
        let local_pos = world_pos % ChunkData::SIZE as i32;
        Some(self.chunks.get(&chunk_pos)?.get_block(local_pos))
    }

    pub fn get_block_or_generate(&mut self, world_pos: Int3) -> Block {
        let chunk_pos = world_pos / ChunkData::SIZE as i32;
        let local_pos = world_pos % ChunkData::SIZE as i32;
        self.generate_chunk(chunk_pos, false);
        self.chunks.get(&chunk_pos).unwrap().get_block(local_pos)
    }
}