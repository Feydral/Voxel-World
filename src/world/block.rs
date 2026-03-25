#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Block {
    Void,
    Air,

    DirtBlock,
    GrassBlock,
    StoneBlock,
    CobbledStoneBlock,

    GrassLayer {
        height: u8,
    },
    Moos,
    MoosLayer {
        height: u8
    },
    Lichen,
}