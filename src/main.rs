use crate::world::{WorldData, chunk::ChunkData};

mod world;
mod math;

fn main() {
    println!("sizeof(ChunkData): {} bytes", std::mem::size_of::<ChunkData>());
    println!("sizeof(WorldData): {} bytes", std::mem::size_of::<WorldData>());
}
