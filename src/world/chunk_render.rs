use crate::{math::numerics::int3::Int3, renderer::types::mesh::Mesh, world::{WorldData, chunk::ChunkData}};

pub fn generate_mesh(position: Int3, world: &WorldData) -> Mesh {
    let chunk = world.get_chunk(position).unwrap();

    for x in 0..ChunkData::SIZE {
        for y in 0..ChunkData::SIZE {
            for z in 0..ChunkData::SIZE {
                let world_pos = position * ChunkData::SIZE as i32 + Int3::new(x as i32, y as i32, z as i32);
                let block = world.get_block(world_pos).unwrap();
            }
        }
    }

    todo!()
}