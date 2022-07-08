#[derive(Clone, Default, Copy, Debug, PartialEq, Eq)]
pub struct DungeonCoordinates {
    pub floor: i32,
    pub col: i32,
    pub row: i32,
}

impl DungeonCoordinates {
    pub fn new(floor: i32, col: i32, row: i32) -> Self {
        Self { floor, col, row }
    }
}