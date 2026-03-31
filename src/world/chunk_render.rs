use crate::{math::numerics::{float2::Float2, float3::Float3, int3::Int3}, renderer::types::mesh::Mesh, world::{WorldData, block::Block, chunk::ChunkData}};

pub fn generate_mesh(position: Int3, world: &WorldData) -> Mesh {
    let mut vertices: Vec<Float3> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut normals: Vec<Float3> = Vec::new();
    let mut uvs: Vec<Float2> = Vec::new();

    for x in 0..ChunkData::SIZE {
        for y in 0..ChunkData::SIZE {
            for z in 0..ChunkData::SIZE {
                let world_pos = position * ChunkData::SIZE as i32 + Int3::new(x as i32, y as i32, z as i32);

                let block = world.get_block(world_pos).unwrap();

                if block == Block::Air { continue; }

                let (x, y, z) = (x as f32, y as f32, z as f32);

                if world.get_block(world_pos + Int3::new(1, 0, 0)).map_or(true, |b| b == Block::Air) {
                    let base = vertices.len() as u32;
                    vertices.extend_from_slice(&[
                        Float3::new(x + 1.0, y + 0.0, z + 0.0),
                        Float3::new(x + 1.0, y + 1.0, z + 0.0),
                        Float3::new(x + 1.0, y + 1.0, z + 1.0),
                        Float3::new(x + 1.0, y + 0.0, z + 1.0),
                    ]);
                    normals.extend_from_slice(&[Float3::new(1.0, 0.0, 0.0); 4]);
                    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                    uvs.extend_from_slice(&[Float2::new(0.0, 0.0), Float2::new(0.0, 1.0), Float2::new(1.0, 1.0), Float2::new(1.0, 0.0)]);
                }
                
                if world.get_block(world_pos + Int3::new(-1, 0, 0)).map_or(true, |b| b == Block::Air) {
                    let base = vertices.len() as u32;
                    vertices.extend_from_slice(&[
                        Float3::new(x + 0.0, y + 0.0, z + 1.0),
                        Float3::new(x + 0.0, y + 1.0, z + 1.0),
                        Float3::new(x + 0.0, y + 1.0, z + 0.0),
                        Float3::new(x + 0.0, y + 0.0, z + 0.0),
                    ]);
                    normals.extend_from_slice(&[Float3::new(-1.0, 0.0, 0.0); 4]);
                    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                    uvs.extend_from_slice(&[Float2::new(0.0, 0.0), Float2::new(0.0, 1.0), Float2::new(1.0, 1.0), Float2::new(1.0, 0.0)]);
                }
                
                if world.get_block(world_pos + Int3::new(0, 1, 0)).map_or(true, |b| b == Block::Air) {
                    let base = vertices.len() as u32;
                    vertices.extend_from_slice(&[
                        Float3::new(x + 0.0, y + 1.0, z + 0.0),
                        Float3::new(x + 0.0, y + 1.0, z + 1.0),
                        Float3::new(x + 1.0, y + 1.0, z + 1.0),
                        Float3::new(x + 1.0, y + 1.0, z + 0.0),
                    ]);
                    normals.extend_from_slice(&[Float3::new(0.0, 1.0, 0.0); 4]);
                    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                    uvs.extend_from_slice(&[Float2::new(0.0, 0.0), Float2::new(0.0, 1.0), Float2::new(1.0, 1.0), Float2::new(1.0, 0.0)]);
                }
                
                if world.get_block(world_pos + Int3::new(0, -1, 0)).map_or(true, |b| b == Block::Air) {
                    let base = vertices.len() as u32;
                    vertices.extend_from_slice(&[
                        Float3::new(x + 1.0, y + 0.0, z + 0.0),
                        Float3::new(x + 1.0, y + 0.0, z + 1.0),
                        Float3::new(x + 0.0, y + 0.0, z + 1.0),
                        Float3::new(x + 0.0, y + 0.0, z + 0.0),
                    ]);
                    normals.extend_from_slice(&[Float3::new(0.0, -1.0, 0.0); 4]);
                    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                    uvs.extend_from_slice(&[Float2::new(0.0, 0.0), Float2::new(0.0, 1.0), Float2::new(1.0, 1.0), Float2::new(1.0, 0.0)]);
                }
                
                if world.get_block(world_pos + Int3::new(0, 0, 1)).map_or(true, |b| b == Block::Air) {
                    let base = vertices.len() as u32;
                    vertices.extend_from_slice(&[
                        Float3::new(x + 1.0, y + 0.0, z + 1.0),
                        Float3::new(x + 1.0, y + 1.0, z + 1.0),
                        Float3::new(x + 0.0, y + 1.0, z + 1.0),
                        Float3::new(x + 0.0, y + 0.0, z + 1.0),
                    ]);
                    normals.extend_from_slice(&[Float3::new(0.0, 0.0, 1.0); 4]);
                    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                    uvs.extend_from_slice(&[Float2::new(0.0, 0.0), Float2::new(0.0, 1.0), Float2::new(1.0, 1.0), Float2::new(1.0, 0.0)]);
                }
                
                if world.get_block(world_pos + Int3::new(0, 0, -1)).map_or(true, |b| b == Block::Air) {
                    let base = vertices.len() as u32;
                    vertices.extend_from_slice(&[
                        Float3::new(x + 0.0, y + 0.0, z + 0.0),
                        Float3::new(x + 0.0, y + 1.0, z + 0.0),
                        Float3::new(x + 1.0, y + 1.0, z + 0.0),
                        Float3::new(x + 1.0, y + 0.0, z + 0.0),
                    ]);
                    normals.extend_from_slice(&[Float3::new(0.0, 0.0, -1.0); 4]);
                    indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                    uvs.extend_from_slice(&[Float2::new(0.0, 0.0), Float2::new(0.0, 1.0), Float2::new(1.0, 1.0), Float2::new(1.0, 0.0)]);
                }
            }
        }
    }

    Mesh::new(vertices, indices, normals, uvs)
}