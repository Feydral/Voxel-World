use crate::{math::numerics::uint3::UInt3, world::block::Block};

pub struct ChunkData {
    blocks: [Block; (Self::SIZE * Self::SIZE * Self::SIZE) as usize],
}

impl ChunkData {
    pub const SIZE: u32 = 32;

    pub fn new() -> Self {
        Self { 
            blocks: [Block::Void; (Self::SIZE * Self::SIZE * Self::SIZE) as usize],
        }
    }

    pub fn get_block(&self, position: UInt3) -> Block {
        let index = (position.x + position.y * Self::SIZE + position.z * Self::SIZE * Self::SIZE) as usize;
        self.blocks[index]
    }
    
    pub fn set_block(&mut self, position: UInt3, block: Block) {
        let index = (position.x + position.y * Self::SIZE + position.z * Self::SIZE * Self::SIZE) as usize;
        self.blocks[index] = block;
    }
}