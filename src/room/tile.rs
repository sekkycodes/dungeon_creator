#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DungeonTile {
    Floor,
    Wall,
    Exit,
    StairsUp,
    StairsDown,
}
