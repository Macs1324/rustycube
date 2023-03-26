#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockId {
    Air,
    Dirt,
    Grass,
    Stone,
    Sand,
    Water,
}

pub struct Block {
    pub id: BlockId,
}
