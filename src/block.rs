#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockId {
    Empty,
    Dirt,
    Grass,
}

pub struct Block {
    pub id: BlockId,
}
