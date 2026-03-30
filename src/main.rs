mod world;
mod math;
mod canvas;
mod input;

use crossterm::event::KeyCode;

use crate::{canvas::Canvas, input::Input, math::numerics::int3::Int3, world::WorldData};

fn main() {
    let mut world = WorldData::new();
    world.create_chunk(Int3::new(0, 0, 0), false);

    let mut canvas = Canvas::new();
    let mut input = Input::new();

    loop {
        let _ = input.update();
        if input.is_key_pressed(KeyCode::Esc) { break; }

        // code here...

        canvas.render();
    }
}
