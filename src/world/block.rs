#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Block {
    Debug,

    Void,
    Air,

    DirtBlock,
    GrassBlock,
    StoneBlock,

    GrassLayer {
        height: u8,
    },

    Moos,
    MoosLayer {
        height: u8
    },

    Lichen,

    SmallGrass,
    Grass,
    TallGrass,
}