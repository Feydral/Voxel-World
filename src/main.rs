mod world;
mod math;
mod canvas;

use crate::{math::numerics::int3::Int3, world::WorldData};

fn main() {
    let mut world = WorldData::new();
    world.create_chunk(Int3::new(0, 0, 0), false);
}
