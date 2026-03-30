mod world;
mod math;
mod canvas;
mod input;

use crossterm::event::KeyCode;

use crate::{canvas::Canvas, input::Input, math::numerics::{int3::Int3, uint3::UInt3}, world::{WorldData, block::Block, chunk::ChunkData}};

fn main() {
    let mut world = WorldData::new();

    let mut canvas = Canvas::new();
    let mut input = Input::new();

    let chunk = world.get_chunk_or_create(Int3::new(0, 0, 0));

    let mut current_slice = 0;

    loop {
        let _ = input.update();
        if input.is_key_pressed(KeyCode::Esc) { break; }

        if input.is_key_down(KeyCode::Up) { current_slice += 1 } 
        if input.is_key_down(KeyCode::Down) { current_slice -= 1 }

        for x in 0..ChunkData::SIZE {
            for y in 0..ChunkData::SIZE {
                let block = chunk.get_block(UInt3::new(x, y, current_slice));

                use math::mathi::rgb_to_u32 as color;

                let color = match block {
                    Block::Air => color(173, 216, 230),
                    Block::DirtBlock => color(139, 101, 68),
                    Block::GrassBlock => color(88, 157, 54),
                    Block::StoneBlock => color(138, 134, 128),
                    _ => color(255, 0, 255)
                };

                canvas.set_pixel(x, y, color);
            }
        }

        canvas.render();
    }
}
