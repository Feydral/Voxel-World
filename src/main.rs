use crate::{math::numerics::int3::Int3, world::WorldData};

mod world;
mod math;

fn main() {
    let mut world = WorldData::new();
    world.create_chunk(Int3::new(0, 0, 0), false);
}
