use crate::{math::numerics::int3::Int3, world::{block::Block, chunk::ChunkData}};

pub fn generate_chunk(position: Int3) -> ChunkData {
    ChunkData::new(position, Block::Air)
}