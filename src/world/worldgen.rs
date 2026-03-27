use crate::{math::numerics::int3::Int3, world::{block::Block, chunk::ChunkData}};

pub fn generate_chunk(position: Int3) -> ChunkData {
    let mut chunk = ChunkData::new(position, Block::Air);

    for x in 0..ChunkData::SIZE {
        for y in 0..ChunkData::SIZE {
            for z in 0..ChunkData::SIZE {
                let world_y = position.y * ChunkData::SIZE as i32 + y as i32;

                let block = if world_y < 0 {
                    Block::StoneBlock
                } else if world_y == 0 {
                    Block::GrassBlock
                } else {
                    Block::Air
                };
                
                chunk.set_block(Int3::new(x as i32, y as i32, z as i32), block);
            }
        }
    }

    chunk
}