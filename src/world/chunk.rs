use crate::{math::numerics::int3::Int3, world::block::Block};

pub struct ChunkData {
    blocks: [Block; (Self::SIZE * Self::SIZE * Self::SIZE) as usize],
    position: Int3,
}

impl ChunkData {
    pub const SIZE: u32 = 32;

    pub fn new(position: Int3, filler: Block) -> Self {
        Self { 
            blocks: [filler; (Self::SIZE * Self::SIZE * Self::SIZE) as usize],
            position,
        }
    }

    pub fn new_empty(position: Int3) -> Self {
        Self { 
            blocks: [Block::Void; (Self::SIZE * Self::SIZE * Self::SIZE) as usize],
            position,
        }
    }

    pub fn get_block(&self, position: Int3) -> Block {
        let index = (position.x + position.y * Self::SIZE as i32 + position.z * Self::SIZE as i32 * Self::SIZE as i32) as usize;
        self.blocks[index]
    }
    
    pub fn set_block(&mut self, position: Int3, block: Block) {
        let index = (position.x + position.y * Self::SIZE as i32 + position.z * Self::SIZE as i32 * Self::SIZE as i32) as usize;
        self.blocks[index] = block;
    }
}